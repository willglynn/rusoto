// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;
use std::io;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>Amazon Device Messaging channel definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ADMChannelRequest {
    /// <p>Client ID as gotten from Amazon</p>
    #[serde(rename = "ClientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>Client secret as gotten from Amazon</p>
    #[serde(rename = "ClientSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// <p>Amazon Device Messaging channel definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ADMChannelResponse {
    /// <p>The ID of the application to which the channel applies.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>When was this segment created</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Not used. Retained for backwards compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>Channel ID. Not used, only for backwards compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Is this channel archived</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>Who last updated this entry</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>Last date this was updated</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>Platform type. Will be &quot;ADM&quot;</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Version of channel</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>ADM Message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ADMMessage {
    /// <p>The action that occurs if the user taps a push notification delivered by the campaign: OPEN<em>APP - Your app launches, or it becomes the foreground app if it has been sent to the background. This is the default action. DEEP</em>LINK - Uses deep linking features in iOS and Android to open your app and display a designated user interface within the app. URL - The default mobile browser on the user&#39;s device launches and opens a web page at the URL you specify. Possible values include: OPEN<em>APP | DEEP</em>LINK | URL</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The message body of the notification, the email body or the text message.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>Optional. Arbitrary string used to indicate multiple messages are logically the same and that ADM is allowed to drop previously enqueued messages in favor of this one.</p>
    #[serde(rename = "ConsolidationKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consolidation_key: Option<String>,
    /// <p>The data payload used for a silent push. This payload is added to the notifications&#39; data.pinpoint.jsonBody&#39; object</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, String>>,
    /// <p>Optional. Number of seconds ADM should retain the message if the device is offline</p>
    #[serde(rename = "ExpiresAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<String>,
    /// <p>The icon image name of the asset saved in your application.</p>
    #[serde(rename = "IconReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_reference: Option<String>,
    /// <p>The URL that points to an image used as the large icon to the notification content view.</p>
    #[serde(rename = "ImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_icon_url: Option<String>,
    /// <p>The URL that points to an image used in the push notification.</p>
    #[serde(rename = "ImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// <p>Optional. Base-64-encoded MD5 checksum of the data parameter. Used to verify data integrity</p>
    #[serde(rename = "MD5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
    /// <p>The Raw JSON formatted string to be used as the payload. This value overrides the message.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>Indicates if the message should display on the users device. Silent pushes can be used for Remote Configuration and Phone Home use cases.</p>
    #[serde(rename = "SilentPush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    /// <p>The URL that points to an image used as the small icon for the notification which will be used to represent the notification in the status bar and content view</p>
    #[serde(rename = "SmallImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image_icon_url: Option<String>,
    /// <p>Indicates a sound to play when the device receives the notification. Supports default, or the filename of a sound resource bundled in the app. Android sound files must reside in /res/raw/</p>
    #[serde(rename = "Sound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    /// <p>Default message substitutions. Can be overridden by individual address substitutions.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The message title that displays above the message on the user&#39;s device.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The URL to open in the user&#39;s mobile browser. Used if the value for Action is URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Apple Push Notification Service channel definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct APNSChannelRequest {
    /// <p>The bundle id used for APNs Tokens.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The distribution certificate from Apple.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>The default authentication method used for APNs.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The certificate private key.</p>
    #[serde(rename = "PrivateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// <p>The team id used for APNs Tokens.</p>
    #[serde(rename = "TeamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// <p>The token key used for APNs Tokens.</p>
    #[serde(rename = "TokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key: Option<String>,
    /// <p>The token key used for APNs Tokens.</p>
    #[serde(rename = "TokenKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_id: Option<String>,
}

/// <p>Apple Distribution Push Notification Service channel definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct APNSChannelResponse {
    /// <p>The ID of the application to which the channel applies.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>When was this segment created</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The default authentication method used for APNs.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Not used. Retained for backwards compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>Indicates whether the channel is configured with a key for APNs token authentication. Provide a token key by setting the TokenKey attribute.</p>
    #[serde(rename = "HasTokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_token_key: Option<bool>,
    /// <p>Channel ID. Not used. Present only for backwards compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Is this channel archived</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>Who last updated this entry</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>Last date this was updated</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The platform type. Will be APNS.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Version of channel</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>APNS Message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct APNSMessage {
    /// <p>The action that occurs if the user taps a push notification delivered by the campaign: OPEN<em>APP - Your app launches, or it becomes the foreground app if it has been sent to the background. This is the default action. DEEP</em>LINK - Uses deep linking features in iOS and Android to open your app and display a designated user interface within the app. URL - The default mobile browser on the user&#39;s device launches and opens a web page at the URL you specify. Possible values include: OPEN<em>APP | DEEP</em>LINK | URL</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>Include this key when you want the system to modify the badge of your app icon. If this key is not included in the dictionary, the badge is not changed. To remove the badge, set the value of this key to 0.</p>
    #[serde(rename = "Badge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<i64>,
    /// <p>The message body of the notification, the email body or the text message.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>Provide this key with a string value that represents the notification&#39;s type. This value corresponds to the value in the identifier property of one of your app&#39;s registered categories.</p>
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>An ID that, if assigned to multiple messages, causes APNs to coalesce the messages into a single push notification instead of delivering each message individually. The value must not exceed 64 bytes. Amazon Pinpoint uses this value to set the apns-collapse-id request header when it sends the message to APNs.</p>
    #[serde(rename = "CollapseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapse_id: Option<String>,
    /// <p>The data payload used for a silent push. This payload is added to the notifications&#39; data.pinpoint.jsonBody&#39; object</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, String>>,
    /// <p>The URL that points to a video used in the push notification.</p>
    #[serde(rename = "MediaUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    /// <p>The preferred authentication method, either &quot;CERTIFICATE&quot; or &quot;TOKEN&quot;</p>
    #[serde(rename = "PreferredAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_authentication_method: Option<String>,
    /// <p>The message priority. Amazon Pinpoint uses this value to set the apns-priority request header when it sends the message to APNs. Accepts the following values:</p>
    ///
    /// <p>&quot;5&quot; - Low priority. Messages might be delayed, delivered in groups, and throttled.</p>
    ///
    /// <p>&quot;10&quot; - High priority. Messages are sent immediately. High priority messages must cause an alert, sound, or badge on the receiving device.</p>
    ///
    /// <p>The default value is &quot;10&quot;.</p>
    ///
    /// <p>The equivalent values for FCM or GCM messages are &quot;normal&quot; and &quot;high&quot;. Amazon Pinpoint accepts these values for APNs messages and converts them.</p>
    ///
    /// <p>For more information about the apns-priority parameter, see Communicating with APNs in the APNs Local and Remote Notification Programming Guide.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// <p>The Raw JSON formatted string to be used as the payload. This value overrides the message.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>Indicates if the message should display on the users device. Silent pushes can be used for Remote Configuration and Phone Home use cases.</p>
    #[serde(rename = "SilentPush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    /// <p>Include this key when you want the system to play a sound. The value of this key is the name of a sound file in your app&#39;s main bundle or in the Library/Sounds folder of your app&#39;s data container. If the sound file cannot be found, or if you specify defaultfor the value, the system plays the default alert sound.</p>
    #[serde(rename = "Sound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    /// <p>Default message substitutions. Can be overridden by individual address substitutions.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>Provide this key with a string value that represents the app-specific identifier for grouping notifications. If you provide a Notification Content app extension, you can use this value to group your notifications together.</p>
    #[serde(rename = "ThreadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /// <p>The length of time (in seconds) that APNs stores and attempts to deliver the message. If the value is 0, APNs does not store the message or attempt to deliver it more than once. Amazon Pinpoint uses this value to set the apns-expiration request header when it sends the message to APNs.</p>
    #[serde(rename = "TimeToLive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<i64>,
    /// <p>The message title that displays above the message on the user&#39;s device.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The URL to open in the user&#39;s mobile browser. Used if the value for Action is URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Apple Development Push Notification Service channel definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct APNSSandboxChannelRequest {
    /// <p>The bundle id used for APNs Tokens.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The distribution certificate from Apple.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>The default authentication method used for APNs.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The certificate private key.</p>
    #[serde(rename = "PrivateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// <p>The team id used for APNs Tokens.</p>
    #[serde(rename = "TeamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// <p>The token key used for APNs Tokens.</p>
    #[serde(rename = "TokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key: Option<String>,
    /// <p>The token key used for APNs Tokens.</p>
    #[serde(rename = "TokenKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_id: Option<String>,
}

/// <p>Apple Development Push Notification Service channel definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct APNSSandboxChannelResponse {
    /// <p>The ID of the application to which the channel applies.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>When was this segment created</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The default authentication method used for APNs.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Not used. Retained for backwards compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>Indicates whether the channel is configured with a key for APNs token authentication. Provide a token key by setting the TokenKey attribute.</p>
    #[serde(rename = "HasTokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_token_key: Option<bool>,
    /// <p>Channel ID. Not used, only for backwards compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Is this channel archived</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>Who last updated this entry</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>Last date this was updated</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The platform type. Will be APNS_SANDBOX.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Version of channel</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Apple VoIP Push Notification Service channel definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct APNSVoipChannelRequest {
    /// <p>The bundle id used for APNs Tokens.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The distribution certificate from Apple.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>The default authentication method used for APNs.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The certificate private key.</p>
    #[serde(rename = "PrivateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// <p>The team id used for APNs Tokens.</p>
    #[serde(rename = "TeamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// <p>The token key used for APNs Tokens.</p>
    #[serde(rename = "TokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key: Option<String>,
    /// <p>The token key used for APNs Tokens.</p>
    #[serde(rename = "TokenKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_id: Option<String>,
}

/// <p>Apple VoIP Push Notification Service channel definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct APNSVoipChannelResponse {
    /// <p>Application id</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>When was this segment created</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The default authentication method used for APNs.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Not used. Retained for backwards compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>If the channel is registered with a token key for authentication.</p>
    #[serde(rename = "HasTokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_token_key: Option<bool>,
    /// <p>Channel ID. Not used, only for backwards compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Is this channel archived</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>Who made the last change</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>Last date this was updated</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The platform type. Will be APNS.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Version of channel</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Apple VoIP Developer Push Notification Service channel definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct APNSVoipSandboxChannelRequest {
    /// <p>The bundle id used for APNs Tokens.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The distribution certificate from Apple.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>The default authentication method used for APNs.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The certificate private key.</p>
    #[serde(rename = "PrivateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// <p>The team id used for APNs Tokens.</p>
    #[serde(rename = "TeamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// <p>The token key used for APNs Tokens.</p>
    #[serde(rename = "TokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key: Option<String>,
    /// <p>The token key used for APNs Tokens.</p>
    #[serde(rename = "TokenKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_id: Option<String>,
}

/// <p>Apple VoIP Developer Push Notification Service channel definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct APNSVoipSandboxChannelResponse {
    /// <p>Application id</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>When was this segment created</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The default authentication method used for APNs.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Not used. Retained for backwards compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>If the channel is registered with a token key for authentication.</p>
    #[serde(rename = "HasTokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_token_key: Option<bool>,
    /// <p>Channel ID. Not used, only for backwards compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Is this channel archived</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>Who made the last change</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>Last date this was updated</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The platform type. Will be APNS.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Version of channel</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Activities for campaign.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ActivitiesResponse {
    /// <p>List of campaign activities</p>
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<ActivityResponse>>,
}

/// <p>Activity definition</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ActivityResponse {
    /// <p>The ID of the application to which the campaign applies.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The ID of the campaign to which the activity applies.</p>
    #[serde(rename = "CampaignId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// <p>The actual time the activity was marked CANCELLED or COMPLETED. Provided in ISO 8601 format.</p>
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// <p>The unique activity ID.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Indicates whether the activity succeeded.</p>
    ///
    /// <p>Valid values: SUCCESS, FAIL</p>
    #[serde(rename = "Result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// <p>The scheduled start time for the activity in ISO 8601 format.</p>
    #[serde(rename = "ScheduledStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_start: Option<String>,
    /// <p>The actual start time of the activity in ISO 8601 format.</p>
    #[serde(rename = "Start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// <p>The state of the activity.</p>
    ///
    /// <p>Valid values: PENDING, INITIALIZING, RUNNING, PAUSED, CANCELLED, COMPLETED</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The total number of endpoints to which the campaign successfully delivered messages.</p>
    #[serde(rename = "SuccessfulEndpointCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_endpoint_count: Option<i64>,
    /// <p>The total number of timezones completed.</p>
    #[serde(rename = "TimezonesCompletedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezones_completed_count: Option<i64>,
    /// <p>The total number of unique timezones present in the segment.</p>
    #[serde(rename = "TimezonesTotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezones_total_count: Option<i64>,
    /// <p>The total number of endpoints to which the campaign attempts to deliver messages.</p>
    #[serde(rename = "TotalEndpointCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_endpoint_count: Option<i64>,
    /// <p>The ID of a variation of the campaign used for A/B testing.</p>
    #[serde(rename = "TreatmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_id: Option<String>,
}

/// <p>Address configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddressConfiguration {
    /// <p>Body override. If specified will override default body.</p>
    #[serde(rename = "BodyOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_override: Option<String>,
    /// <p>The channel type.</p>
    ///
    /// <p>Valid values: GCM | APNS | APNS<em>SANDBOX | APNS</em>VOIP | APNS<em>VOIP</em>SANDBOX | ADM | SMS | EMAIL | BAIDU</p>
    #[serde(rename = "ChannelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    /// <p>A map of custom attributes to attributes to be attached to the message for this address. This payload is added to the push notification&#39;s &#39;data.pinpoint&#39; object or added to the email/sms delivery receipt event attributes.</p>
    #[serde(rename = "Context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<::std::collections::HashMap<String, String>>,
    /// <p>The Raw JSON formatted string to be used as the payload. This value overrides the message.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>A map of substitution values for the message to be merged with the DefaultMessage&#39;s substitutions. Substitutions on this map take precedence over the all other substitutions.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>Title override. If specified will override default title if applicable.</p>
    #[serde(rename = "TitleOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_override: Option<String>,
}

/// <p>Application Response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ApplicationResponse {
    /// <p>The unique application ID.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The display name of the application.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Application settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ApplicationSettingsResource {
    /// <p>The unique ID for the application.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>Default campaign hook.</p>
    #[serde(rename = "CampaignHook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_hook: Option<CampaignHook>,
    /// <p>The date that the settings were last updated in ISO 8601 format.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The default campaign limits for the app. These limits apply to each campaign for the app, unless the campaign overrides the default with limits of its own.</p>
    #[serde(rename = "Limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<CampaignLimits>,
    /// <p>The default quiet time for the app. Each campaign for this app sends no messages during this time unless the campaign overrides the default with a quiet time of its own.</p>
    #[serde(rename = "QuietTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_time: Option<QuietTime>,
}

/// <p>Get Applications Result.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ApplicationsResponse {
    /// <p>List of applications returned in this page.</p>
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<ApplicationResponse>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Custom attibute dimension</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeDimension {
    /// <p>The type of dimension:
    /// INCLUSIVE - Endpoints that match the criteria are included in the segment.
    /// EXCLUSIVE - Endpoints that match the criteria are excluded from the segment.</p>
    #[serde(rename = "AttributeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<String>,
    /// <p>The criteria values for the segment dimension. Endpoints with matching attribute values are included or excluded from the segment, depending on the setting for Type.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AttributesResource {
    /// <p>The unique ID for the application.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The attribute type for the application.</p>
    #[serde(rename = "AttributeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<String>,
    /// <p>The attributes for the application.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
}

/// <p>Baidu Cloud Push credentials</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BaiduChannelRequest {
    /// <p>Platform credential API key from Baidu.</p>
    #[serde(rename = "ApiKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Platform credential Secret key from Baidu.</p>
    #[serde(rename = "SecretKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
}

/// <p>Baidu Cloud Messaging channel definition</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BaiduChannelResponse {
    /// <p>Application id</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>When was this segment created</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The Baidu API key from Baidu.</p>
    #[serde(rename = "Credential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Not used. Retained for backwards compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>Channel ID. Not used, only for backwards compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Is this channel archived</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>Who made the last change</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>Last date this was updated</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The platform type. Will be BAIDU</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Version of channel</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Baidu Message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BaiduMessage {
    /// <p>The action that occurs if the user taps a push notification delivered by the campaign: OPEN<em>APP - Your app launches, or it becomes the foreground app if it has been sent to the background. This is the default action. DEEP</em>LINK - Uses deep linking features in iOS and Android to open your app and display a designated user interface within the app. URL - The default mobile browser on the user&#39;s device launches and opens a web page at the URL you specify. Possible values include: OPEN<em>APP | DEEP</em>LINK | URL</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The message body of the notification, the email body or the text message.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The data payload used for a silent push. This payload is added to the notifications&#39; data.pinpoint.jsonBody&#39; object</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, String>>,
    /// <p>The icon image name of the asset saved in your application.</p>
    #[serde(rename = "IconReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_reference: Option<String>,
    /// <p>The URL that points to an image used as the large icon to the notification content view.</p>
    #[serde(rename = "ImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_icon_url: Option<String>,
    /// <p>The URL that points to an image used in the push notification.</p>
    #[serde(rename = "ImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// <p>The Raw JSON formatted string to be used as the payload. This value overrides the message.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>Indicates if the message should display on the users device. Silent pushes can be used for Remote Configuration and Phone Home use cases.</p>
    #[serde(rename = "SilentPush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    /// <p>The URL that points to an image used as the small icon for the notification which will be used to represent the notification in the status bar and content view</p>
    #[serde(rename = "SmallImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image_icon_url: Option<String>,
    /// <p>Indicates a sound to play when the device receives the notification. Supports default, or the filename of a sound resource bundled in the app. Android sound files must reside in /res/raw/</p>
    #[serde(rename = "Sound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    /// <p>Default message substitutions. Can be overridden by individual address substitutions.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>This parameter specifies how long (in seconds) the message should be kept in Baidu storage if the device is offline. The and the default value and the maximum time to live supported is 7 days (604800 seconds)</p>
    #[serde(rename = "TimeToLive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<i64>,
    /// <p>The message title that displays above the message on the user&#39;s device.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The URL to open in the user&#39;s mobile browser. Used if the value for Action is URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>The email message configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CampaignEmailMessage {
    /// <p>The email text body.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The email address used to send the email from. Defaults to use FromAddress specified in the Email Channel.</p>
    #[serde(rename = "FromAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    /// <p>The email html body.</p>
    #[serde(rename = "HtmlBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_body: Option<String>,
    /// <p>The email title (Or subject).</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// <p>Campaign hook information.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CampaignHook {
    /// <p>Lambda function name or arn to be called for delivery</p>
    #[serde(rename = "LambdaFunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_name: Option<String>,
    /// <p>What mode Lambda should be invoked in.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p>Web URL to call for hook. If the URL has authentication specified it will be added as authentication to the request</p>
    #[serde(rename = "WebUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
}

/// <p>Campaign Limits are used to limit the number of messages that can be sent to a user.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CampaignLimits {
    /// <p>The maximum number of messages that the campaign can send daily.</p>
    #[serde(rename = "Daily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily: Option<i64>,
    /// <p>The length of time (in seconds) that the campaign can run before it ends and message deliveries stop. This duration begins at the scheduled start time for the campaign. The minimum value is 60.</p>
    #[serde(rename = "MaximumDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_duration: Option<i64>,
    /// <p>The number of messages that the campaign can send per second. The minimum value is 50, and the maximum is 20000.</p>
    #[serde(rename = "MessagesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_per_second: Option<i64>,
    /// <p>The maximum total number of messages that the campaign can send.</p>
    #[serde(rename = "Total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

/// <p>Campaign definition</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CampaignResponse {
    /// <p>Treatments that are defined in addition to the default treatment.</p>
    #[serde(rename = "AdditionalTreatments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_treatments: Option<Vec<TreatmentResource>>,
    /// <p>The ID of the application to which the campaign applies.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date the campaign was created in ISO 8601 format.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The status of the campaign&#39;s default treatment. Only present for A/B test campaigns.</p>
    #[serde(rename = "DefaultState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_state: Option<CampaignState>,
    /// <p>A description of the campaign.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The allocated percentage of end users who will not receive messages from this campaign.</p>
    #[serde(rename = "HoldoutPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holdout_percent: Option<i64>,
    /// <p>Campaign hook information.</p>
    #[serde(rename = "Hook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook: Option<CampaignHook>,
    /// <p>The unique campaign ID.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Indicates whether the campaign is paused. A paused campaign does not send messages unless you resume it by setting IsPaused to false.</p>
    #[serde(rename = "IsPaused")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_paused: Option<bool>,
    /// <p>The date the campaign was last updated in ISO 8601 format.  </p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The campaign limits settings.</p>
    #[serde(rename = "Limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<CampaignLimits>,
    /// <p>The message configuration settings.</p>
    #[serde(rename = "MessageConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_configuration: Option<MessageConfiguration>,
    /// <p>The custom name of the campaign.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The campaign schedule.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    /// <p>The ID of the segment to which the campaign sends messages.</p>
    #[serde(rename = "SegmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    /// <p>The version of the segment to which the campaign sends messages.</p>
    #[serde(rename = "SegmentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_version: Option<i64>,
    /// <p>The campaign status.</p>
    ///
    /// <p>An A/B test campaign will have a status of COMPLETED only when all treatments have a status of COMPLETED.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<CampaignState>,
    /// <p>A custom description for the treatment.</p>
    #[serde(rename = "TreatmentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_description: Option<String>,
    /// <p>The custom name of a variation of the campaign used for A/B testing.</p>
    #[serde(rename = "TreatmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_name: Option<String>,
    /// <p>The campaign version number.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>SMS message configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CampaignSmsMessage {
    /// <p>The SMS text body.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>Is this is a transactional SMS message, otherwise a promotional message.</p>
    #[serde(rename = "MessageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// <p>Sender ID of sent message.</p>
    #[serde(rename = "SenderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
}

/// <p>State of the Campaign</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CampaignState {
    /// <p>The status of the campaign, or the status of a treatment that belongs to an A/B test campaign.</p>
    ///
    /// <p>Valid values: SCHEDULED, EXECUTING, PENDING<em>NEXT</em>RUN, COMPLETED, PAUSED</p>
    #[serde(rename = "CampaignStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_status: Option<String>,
}

/// <p>List of available campaigns.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CampaignsResponse {
    /// <p>A list of campaigns.</p>
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<CampaignResponse>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Base definition for channel response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ChannelResponse {
    /// <p>Application id</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>When was this segment created</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Not used. Retained for backwards compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>Channel ID. Not used, only for backwards compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Is this channel archived</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>Who made the last change</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>Last date this was updated</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>Version of channel</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Get channels definition</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ChannelsResponse {
    /// <p>A map of channels, with the ChannelType as the key and the Channel as the value.</p>
    #[serde(rename = "Channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<::std::collections::HashMap<String, ChannelResponse>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAppRequest {
    #[serde(rename = "CreateApplicationRequest")]
    pub create_application_request: CreateApplicationRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateAppResponse {
    #[serde(rename = "ApplicationResponse")]
    pub application_response: ApplicationResponse,
}

/// <p>Application Request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateApplicationRequest {
    /// <p>The display name of the application. Used in the Amazon Pinpoint console.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCampaignRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "WriteCampaignRequest")]
    pub write_campaign_request: WriteCampaignRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateCampaignResponse {
    #[serde(rename = "CampaignResponse")]
    pub campaign_response: CampaignResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateExportJobRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "ExportJobRequest")]
    pub export_job_request: ExportJobRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateExportJobResponse {
    #[serde(rename = "ExportJobResponse")]
    pub export_job_response: ExportJobResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateImportJobRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "ImportJobRequest")]
    pub import_job_request: ImportJobRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateImportJobResponse {
    #[serde(rename = "ImportJobResponse")]
    pub import_job_response: ImportJobResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSegmentRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "WriteSegmentRequest")]
    pub write_segment_request: WriteSegmentRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateSegmentResponse {
    #[serde(rename = "SegmentResponse")]
    pub segment_response: SegmentResponse,
}

/// <p>Default Message across push notification, email, and sms.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DefaultMessage {
    /// <p>The message body of the notification, the email body or the text message.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>Default message substitutions. Can be overridden by individual address substitutions.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
}

/// <p>Default Push Notification Message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DefaultPushNotificationMessage {
    /// <p>The action that occurs if the user taps a push notification delivered by the campaign: OPEN<em>APP - Your app launches, or it becomes the foreground app if it has been sent to the background. This is the default action. DEEP</em>LINK - Uses deep linking features in iOS and Android to open your app and display a designated user interface within the app. URL - The default mobile browser on the user&#39;s device launches and opens a web page at the URL you specify. Possible values include: OPEN<em>APP | DEEP</em>LINK | URL</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The message body of the notification, the email body or the text message.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The data payload used for a silent push. This payload is added to the notifications&#39; data.pinpoint.jsonBody&#39; object</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, String>>,
    /// <p>Indicates if the message should display on the users device. Silent pushes can be used for Remote Configuration and Phone Home use cases.</p>
    #[serde(rename = "SilentPush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    /// <p>Default message substitutions. Can be overridden by individual address substitutions.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The message title that displays above the message on the user&#39;s device.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The URL to open in the user&#39;s mobile browser. Used if the value for Action is URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAdmChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteAdmChannelResponse {
    #[serde(rename = "ADMChannelResponse")]
    pub adm_channel_response: ADMChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApnsChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteApnsChannelResponse {
    #[serde(rename = "APNSChannelResponse")]
    pub apns_channel_response: APNSChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApnsSandboxChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteApnsSandboxChannelResponse {
    #[serde(rename = "APNSSandboxChannelResponse")]
    pub apns_sandbox_channel_response: APNSSandboxChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApnsVoipChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteApnsVoipChannelResponse {
    #[serde(rename = "APNSVoipChannelResponse")]
    pub apns_voip_channel_response: APNSVoipChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApnsVoipSandboxChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteApnsVoipSandboxChannelResponse {
    #[serde(rename = "APNSVoipSandboxChannelResponse")]
    pub apns_voip_sandbox_channel_response: APNSVoipSandboxChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAppRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteAppResponse {
    #[serde(rename = "ApplicationResponse")]
    pub application_response: ApplicationResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteBaiduChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteBaiduChannelResponse {
    #[serde(rename = "BaiduChannelResponse")]
    pub baidu_channel_response: BaiduChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteCampaignRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the campaign.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteCampaignResponse {
    #[serde(rename = "CampaignResponse")]
    pub campaign_response: CampaignResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteEmailChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteEmailChannelResponse {
    #[serde(rename = "EmailChannelResponse")]
    pub email_channel_response: EmailChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteEndpointRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the endpoint.</p>
    #[serde(rename = "EndpointId")]
    pub endpoint_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteEndpointResponse {
    #[serde(rename = "EndpointResponse")]
    pub endpoint_response: EndpointResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteEventStreamRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteEventStreamResponse {
    #[serde(rename = "EventStream")]
    pub event_stream: EventStream,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteGcmChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteGcmChannelResponse {
    #[serde(rename = "GCMChannelResponse")]
    pub gcm_channel_response: GCMChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSegmentRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the segment.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteSegmentResponse {
    #[serde(rename = "SegmentResponse")]
    pub segment_response: SegmentResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSmsChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteSmsChannelResponse {
    #[serde(rename = "SMSChannelResponse")]
    pub sms_channel_response: SMSChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUserEndpointsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteUserEndpointsResponse {
    #[serde(rename = "EndpointsResponse")]
    pub endpoints_response: EndpointsResponse,
}

/// <p>Message definitions for the default message and any messages that are tailored for specific channels.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DirectMessageConfiguration {
    /// <p>The message to ADM channels. Overrides the default push notification message.</p>
    #[serde(rename = "ADMMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adm_message: Option<ADMMessage>,
    /// <p>The message to APNS channels. Overrides the default push notification message.</p>
    #[serde(rename = "APNSMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apns_message: Option<APNSMessage>,
    /// <p>The message to Baidu GCM channels. Overrides the default push notification message.</p>
    #[serde(rename = "BaiduMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baidu_message: Option<BaiduMessage>,
    /// <p>The default message for all channels.</p>
    #[serde(rename = "DefaultMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_message: Option<DefaultMessage>,
    /// <p>The default push notification message for all push channels.</p>
    #[serde(rename = "DefaultPushNotificationMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_push_notification_message: Option<DefaultPushNotificationMessage>,
    /// <p>The message to GCM channels. Overrides the default push notification message.</p>
    #[serde(rename = "GCMMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcm_message: Option<GCMMessage>,
    /// <p>The message to SMS channels. Overrides the default message.</p>
    #[serde(rename = "SMSMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_message: Option<SMSMessage>,
}

/// <p>Email Channel Request</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EmailChannelRequest {
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The email address used to send emails from.</p>
    #[serde(rename = "FromAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    /// <p>The ARN of an identity verified with SES.</p>
    #[serde(rename = "Identity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// <p>The ARN of an IAM Role used to submit events to Mobile Analytics&#39; event ingestion service</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>Email Channel Response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EmailChannelResponse {
    /// <p>The unique ID of the application to which the email channel belongs.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date that the settings were last updated in ISO 8601 format.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The email address used to send emails from.</p>
    #[serde(rename = "FromAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    /// <p>Not used. Retained for backwards compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>Channel ID. Not used, only for backwards compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The ARN of an identity verified with SES.</p>
    #[serde(rename = "Identity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// <p>Is this channel archived</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>Who last updated this entry</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>Last date this was updated</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>Messages per second that can be sent</p>
    #[serde(rename = "MessagesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_per_second: Option<i64>,
    /// <p>Platform type. Will be &quot;EMAIL&quot;</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The ARN of an IAM Role used to submit events to Mobile Analytics&#39; event ingestion service</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>Version of channel</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Endpoint update request</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EndpointBatchItem {
    /// <p>The destination for messages that you send to this endpoint. The address varies by channel. For mobile push channels, use the token provided by the push notification service, such as the APNs device token or the FCM registration token. For the SMS channel, use a phone number in E.164 format, such as +1206XXX5550100. For the email channel, use an email address.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>Custom attributes that describe the endpoint by associating a name with an array of values. For example, an attribute named &quot;interests&quot; might have the values [&quot;science&quot;, &quot;politics&quot;, &quot;travel&quot;]. You can use these attributes as selection criteria when you create a segment of users to engage with a messaging campaign.</p>
    ///
    /// <p>The following characters are not recommended in attribute names: # : ? \ /. The Amazon Pinpoint console does not display attributes that include these characters in the name. This limitation does not apply to attribute values.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The channel type.</p>
    ///
    /// <p>Valid values: GCM | APNS | APNS<em>SANDBOX | APNS</em>VOIP | APNS<em>VOIP</em>SANDBOX | ADM | SMS | EMAIL | BAIDU</p>
    #[serde(rename = "ChannelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    /// <p>The endpoint demographic attributes.</p>
    #[serde(rename = "Demographic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographic: Option<EndpointDemographic>,
    /// <p>The last time the endpoint was updated. Provided in ISO 8601 format.</p>
    #[serde(rename = "EffectiveDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// <p>Unused.</p>
    #[serde(rename = "EndpointStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_status: Option<String>,
    /// <p>The unique Id for the Endpoint in the batch.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The endpoint location attributes.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<EndpointLocation>,
    /// <p>Custom metrics that your app reports to Amazon Pinpoint.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, f64>>,
    /// <p>Indicates whether a user has opted out of receiving messages with one of the following values:</p>
    ///
    /// <p>ALL - User has opted out of all messages.</p>
    ///
    /// <p>NONE - Users has not opted out and receives all messages.</p>
    #[serde(rename = "OptOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_out: Option<String>,
    /// <p>The unique ID for the most recent request to update the endpoint.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>Custom user-specific attributes that your app reports to Amazon Pinpoint.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<EndpointUser>,
}

/// <p>Endpoint batch update request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EndpointBatchRequest {
    /// <p>List of items to update. Maximum 100 items</p>
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<EndpointBatchItem>>,
}

/// <p>Endpoint demographic data</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointDemographic {
    /// <p>The version of the application associated with the endpoint.</p>
    #[serde(rename = "AppVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    /// <p>The endpoint locale in the following format: The ISO 639-1 alpha-2 code, followed by an underscore, followed by an ISO 3166-1 alpha-2 value.</p>
    #[serde(rename = "Locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>The endpoint make, such as such as Apple or Samsung.</p>
    #[serde(rename = "Make")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,
    /// <p>The endpoint model, such as iPhone.</p>
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// <p>The endpoint model version.</p>
    #[serde(rename = "ModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
    /// <p>The endpoint platform, such as ios or android.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The endpoint platform version.</p>
    #[serde(rename = "PlatformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>The timezone of the endpoint. Specified as a tz database value, such as Americas/Los_Angeles.</p>
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// <p>Endpoint location data</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointLocation {
    /// <p>The city where the endpoint is located.</p>
    #[serde(rename = "City")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// <p>Country according to ISO 3166-1 Alpha-2 codes. For example, US.</p>
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// <p>The latitude of the endpoint location. Rounded to one decimal (Roughly corresponding to a mile).</p>
    #[serde(rename = "Latitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// <p>The longitude of the endpoint location. Rounded to one decimal (Roughly corresponding to a mile).</p>
    #[serde(rename = "Longitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    /// <p>The postal code or zip code of the endpoint.</p>
    #[serde(rename = "PostalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// <p>The region of the endpoint location. For example, corresponds to a state in US.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// <p>The result from sending a message to an endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EndpointMessageResult {
    /// <p>Address that endpoint message was delivered to.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>Delivery status of message.</p>
    #[serde(rename = "DeliveryStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<String>,
    /// <p>Unique message identifier associated with the message that was sent.</p>
    #[serde(rename = "MessageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// <p>Downstream service status code.</p>
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
    /// <p>Status message for message delivery.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>If token was updated as part of delivery. (This is GCM Specific)</p>
    #[serde(rename = "UpdatedToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_token: Option<String>,
}

/// <p>Endpoint update request</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EndpointRequest {
    /// <p>The destination for messages that you send to this endpoint. The address varies by channel. For mobile push channels, use the token provided by the push notification service, such as the APNs device token or the FCM registration token. For the SMS channel, use a phone number in E.164 format, such as +1206XXX5550100. For the email channel, use an email address.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>Custom attributes that describe the endpoint by associating a name with an array of values. For example, an attribute named &quot;interests&quot; might have the values [&quot;science&quot;, &quot;politics&quot;, &quot;travel&quot;]. You can use these attributes as selection criteria when you create a segment of users to engage with a messaging campaign.</p>
    ///
    /// <p>The following characters are not recommended in attribute names: # : ? \ /. The Amazon Pinpoint console does not display attributes that include these characters in the name. This limitation does not apply to attribute values.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The channel type.</p>
    ///
    /// <p>Valid values: GCM | APNS | APNS<em>SANDBOX | APNS</em>VOIP | APNS<em>VOIP</em>SANDBOX | ADM | SMS | EMAIL | BAIDU</p>
    #[serde(rename = "ChannelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    /// <p>The endpoint demographic attributes.</p>
    #[serde(rename = "Demographic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographic: Option<EndpointDemographic>,
    /// <p>The last time the endpoint was updated. Provided in ISO 8601 format.</p>
    #[serde(rename = "EffectiveDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// <p>Unused.</p>
    #[serde(rename = "EndpointStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_status: Option<String>,
    /// <p>The endpoint location attributes.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<EndpointLocation>,
    /// <p>Custom metrics that your app reports to Amazon Pinpoint.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, f64>>,
    /// <p>Indicates whether a user has opted out of receiving messages with one of the following values:</p>
    ///
    /// <p>ALL - User has opted out of all messages.</p>
    ///
    /// <p>NONE - Users has not opted out and receives all messages.</p>
    #[serde(rename = "OptOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_out: Option<String>,
    /// <p>The unique ID for the most recent request to update the endpoint.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>Custom user-specific attributes that your app reports to Amazon Pinpoint.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<EndpointUser>,
}

/// <p>Endpoint response</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EndpointResponse {
    /// <p>The address or token of the endpoint as provided by your push provider (e.g. DeviceToken or RegistrationId).</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>The ID of the application associated with the endpoint.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>Custom attributes that describe the endpoint by associating a name with an array of values. For example, an attribute named &quot;interests&quot; might have the values [&quot;science&quot;, &quot;politics&quot;, &quot;travel&quot;]. You can use these attributes as selection criteria when you create a segment of users to engage with a messaging campaign.</p>
    ///
    /// <p>The following characters are not recommended in attribute names: # : ? \ /. The Amazon Pinpoint console does not display attributes that include these characters in the name. This limitation does not apply to attribute values.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The channel type.</p>
    ///
    /// <p>Valid values: GCM | APNS | APNS<em>SANDBOX | APNS</em>VOIP | APNS<em>VOIP</em>SANDBOX | ADM | SMS | EMAIL | BAIDU</p>
    #[serde(rename = "ChannelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    /// <p>A number from 0 - 99 that represents the cohort the endpoint is assigned to. Endpoints are grouped into cohorts randomly, and each cohort contains approximately 1 percent of the endpoints for an app. Amazon Pinpoint assigns cohorts to the holdout or treatment allocations for a campaign.</p>
    #[serde(rename = "CohortId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cohort_id: Option<String>,
    /// <p>The last time the endpoint was created. Provided in ISO 8601 format.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The endpoint demographic attributes.</p>
    #[serde(rename = "Demographic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographic: Option<EndpointDemographic>,
    /// <p>The last time the endpoint was updated. Provided in ISO 8601 format.</p>
    #[serde(rename = "EffectiveDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// <p>Unused.</p>
    #[serde(rename = "EndpointStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_status: Option<String>,
    /// <p>The unique ID that you assigned to the endpoint. The ID should be a globally unique identifier (GUID) to ensure that it is unique compared to all other endpoints for the application.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The endpoint location attributes.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<EndpointLocation>,
    /// <p>Custom metrics that your app reports to Amazon Pinpoint.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, f64>>,
    /// <p>Indicates whether a user has opted out of receiving messages with one of the following values:</p>
    ///
    /// <p>ALL - User has opted out of all messages.</p>
    ///
    /// <p>NONE - Users has not opted out and receives all messages.</p>
    #[serde(rename = "OptOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_out: Option<String>,
    /// <p>The unique ID for the most recent request to update the endpoint.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>Custom user-specific attributes that your app reports to Amazon Pinpoint.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<EndpointUser>,
}

/// <p>Endpoint send configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EndpointSendConfiguration {
    /// <p>Body override. If specified will override default body.</p>
    #[serde(rename = "BodyOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_override: Option<String>,
    /// <p>A map of custom attributes to attributes to be attached to the message for this address. This payload is added to the push notification&#39;s &#39;data.pinpoint&#39; object or added to the email/sms delivery receipt event attributes.</p>
    #[serde(rename = "Context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<::std::collections::HashMap<String, String>>,
    /// <p>The Raw JSON formatted string to be used as the payload. This value overrides the message.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>A map of substitution values for the message to be merged with the DefaultMessage&#39;s substitutions. Substitutions on this map take precedence over the all other substitutions.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>Title override. If specified will override default title if applicable.</p>
    #[serde(rename = "TitleOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_override: Option<String>,
}

/// <p>Endpoint user specific custom userAttributes</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointUser {
    /// <p>Custom attributes that describe an end user by associating a name with an array of values. For example, an attribute named &quot;interests&quot; might have the values [&quot;science&quot;, &quot;politics&quot;, &quot;travel&quot;]. You can use these attributes as selection criteria when you create a segment of users to engage with a messaging campaign.</p>
    ///
    /// <p>The following characters are not recommended in attribute names: # : ? \ /. The Amazon Pinpoint console does not display attributes that include these characters in the name. This limitation does not apply to attribute values.</p>
    #[serde(rename = "UserAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attributes: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The unique ID of the user.</p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// <p>List of endpoints</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EndpointsResponse {
    /// <p>The list of endpoints.</p>
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<EndpointResponse>>,
}

/// <p>Model for an event publishing subscription export.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EventStream {
    /// <p>The ID of the application from which events should be published.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon Kinesis stream or Firehose delivery stream to which you want to publish events.
    /// Firehose ARN: arn:aws:firehose:REGION:ACCOUNT<em>ID:deliverystream/STREAM</em>NAME
    /// Kinesis ARN: arn:aws:kinesis:REGION:ACCOUNT<em>ID:stream/STREAM</em>NAME</p>
    #[serde(rename = "DestinationStreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_stream_arn: Option<String>,
    /// <p>DEPRECATED. Your AWS account ID, which you assigned to the ExternalID key in an IAM trust policy. Used by Amazon Pinpoint to assume an IAM role. This requirement is removed, and external IDs are not recommended for IAM roles assumed by Amazon Pinpoint.</p>
    #[serde(rename = "ExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The date the event stream was last updated in ISO 8601 format.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The IAM user who last modified the event stream.</p>
    #[serde(rename = "LastUpdatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<String>,
    /// <p>The IAM role that authorizes Amazon Pinpoint to publish events to the stream in your account.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>Export job request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ExportJobRequest {
    /// <p>The Amazon Resource Name (ARN) of an IAM role that grants Amazon Pinpoint access to the Amazon S3 location that endpoints will be exported to.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>A URL that points to the location within an Amazon S3 bucket that will receive the export. The location is typically a folder with multiple files.</p>
    ///
    /// <p>The URL should follow this format: s3://bucket-name/folder-name/</p>
    ///
    /// <p>Amazon Pinpoint will export endpoints to this location.</p>
    #[serde(rename = "S3UrlPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_url_prefix: Option<String>,
    /// <p>The ID of the segment to export endpoints from. If not present, Amazon Pinpoint exports all of the endpoints that belong to the application.</p>
    #[serde(rename = "SegmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    /// <p>The version of the segment to export if specified.</p>
    #[serde(rename = "SegmentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_version: Option<i64>,
}

/// <p>Export job resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ExportJobResource {
    /// <p>The Amazon Resource Name (ARN) of an IAM role that grants Amazon Pinpoint access to the Amazon S3 location that endpoints will be exported to.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>A URL that points to the location within an Amazon S3 bucket that will receive the export. The location is typically a folder with multiple files.</p>
    ///
    /// <p>The URL should follow this format: s3://bucket-name/folder-name/</p>
    ///
    /// <p>Amazon Pinpoint will export endpoints to this location.</p>
    #[serde(rename = "S3UrlPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_url_prefix: Option<String>,
    /// <p>The ID of the segment to export endpoints from. If not present, Amazon Pinpoint exports all of the endpoints that belong to the application.</p>
    #[serde(rename = "SegmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    /// <p>The version of the segment to export if specified.</p>
    #[serde(rename = "SegmentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_version: Option<i64>,
}

/// <p>Export job response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ExportJobResponse {
    /// <p>The unique ID of the application to which the job applies.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The number of pieces that have successfully completed as of the time of the request.</p>
    #[serde(rename = "CompletedPieces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_pieces: Option<i64>,
    /// <p>The date the job completed in ISO 8601 format.</p>
    #[serde(rename = "CompletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<String>,
    /// <p>The date the job was created in ISO 8601 format.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The export job settings.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<ExportJobResource>,
    /// <p>The number of pieces that failed to be processed as of the time of the request.</p>
    #[serde(rename = "FailedPieces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_pieces: Option<i64>,
    /// <p>Provides up to 100 of the first failed entries for the job, if any exist.</p>
    #[serde(rename = "Failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<String>>,
    /// <p>The unique ID of the job.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The status of the job.
    /// Valid values: CREATED, INITIALIZING, PROCESSING, COMPLETING, COMPLETED, FAILING, FAILED</p>
    ///
    /// <p>The job status is FAILED if one or more pieces failed.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>The number of endpoints that were not processed; for example, because of syntax errors.</p>
    #[serde(rename = "TotalFailures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_failures: Option<i64>,
    /// <p>The total number of pieces that must be processed to finish the job. Each piece is an approximately equal portion of the endpoints.</p>
    #[serde(rename = "TotalPieces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pieces: Option<i64>,
    /// <p>The number of endpoints that were processed by the job.</p>
    #[serde(rename = "TotalProcessed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_processed: Option<i64>,
    /// <p>The job type. Will be &#39;EXPORT&#39;.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Export job list.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ExportJobsResponse {
    /// <p>A list of export jobs for the application.</p>
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<ExportJobResponse>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Google Cloud Messaging credentials</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GCMChannelRequest {
    /// <p>Platform credential API key from Google.</p>
    #[serde(rename = "ApiKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// <p>Google Cloud Messaging channel definition</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GCMChannelResponse {
    /// <p>The ID of the application to which the channel applies.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>When was this segment created</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The GCM API key from Google.</p>
    #[serde(rename = "Credential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Not used. Retained for backwards compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>Channel ID. Not used. Present only for backwards compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Is this channel archived</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>Who last updated this entry</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>Last date this was updated</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The platform type. Will be GCM</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Version of channel</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>GCM Message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GCMMessage {
    /// <p>The action that occurs if the user taps a push notification delivered by the campaign: OPEN<em>APP - Your app launches, or it becomes the foreground app if it has been sent to the background. This is the default action. DEEP</em>LINK - Uses deep linking features in iOS and Android to open your app and display a designated user interface within the app. URL - The default mobile browser on the user&#39;s device launches and opens a web page at the URL you specify. Possible values include: OPEN<em>APP | DEEP</em>LINK | URL</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The message body of the notification, the email body or the text message.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>This parameter identifies a group of messages (e.g., with collapse_key: &quot;Updates Available&quot;) that can be collapsed, so that only the last message gets sent when delivery can be resumed. This is intended to avoid sending too many of the same messages when the device comes back online or becomes active.</p>
    #[serde(rename = "CollapseKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapse_key: Option<String>,
    /// <p>The data payload used for a silent push. This payload is added to the notifications&#39; data.pinpoint.jsonBody&#39; object</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, String>>,
    /// <p>The icon image name of the asset saved in your application.</p>
    #[serde(rename = "IconReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_reference: Option<String>,
    /// <p>The URL that points to an image used as the large icon to the notification content view.</p>
    #[serde(rename = "ImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_icon_url: Option<String>,
    /// <p>The URL that points to an image used in the push notification.</p>
    #[serde(rename = "ImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// <p>The message priority. Amazon Pinpoint uses this value to set the FCM or GCM priority parameter when it sends the message. Accepts the following values:</p>
    ///
    /// <p>&quot;Normal&quot; - Messages might be delayed. Delivery is optimized for battery usage on the receiving device. Use normal priority unless immediate delivery is required.</p>
    ///
    /// <p>&quot;High&quot; - Messages are sent immediately and might wake a sleeping device.</p>
    ///
    /// <p>The equivalent values for APNs messages are &quot;5&quot; and &quot;10&quot;. Amazon Pinpoint accepts these values here and converts them.</p>
    ///
    /// <p>For more information, see About FCM Messages in the Firebase documentation.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// <p>The Raw JSON formatted string to be used as the payload. This value overrides the message.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>This parameter specifies the package name of the application where the registration tokens must match in order to receive the message.</p>
    #[serde(rename = "RestrictedPackageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_package_name: Option<String>,
    /// <p>Indicates if the message should display on the users device. Silent pushes can be used for Remote Configuration and Phone Home use cases.</p>
    #[serde(rename = "SilentPush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    /// <p>The URL that points to an image used as the small icon for the notification which will be used to represent the notification in the status bar and content view</p>
    #[serde(rename = "SmallImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image_icon_url: Option<String>,
    /// <p>Indicates a sound to play when the device receives the notification. Supports default, or the filename of a sound resource bundled in the app. Android sound files must reside in /res/raw/</p>
    #[serde(rename = "Sound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    /// <p>Default message substitutions. Can be overridden by individual address substitutions.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The length of time (in seconds) that FCM or GCM stores and attempts to deliver the message. If unspecified, the value defaults to the maximum, which is 2,419,200 seconds (28 days). Amazon Pinpoint uses this value to set the FCM or GCM time<em>to</em>live parameter.</p>
    #[serde(rename = "TimeToLive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<i64>,
    /// <p>The message title that displays above the message on the user&#39;s device.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The URL to open in the user&#39;s mobile browser. Used if the value for Action is URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>GPS coordinates</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GPSCoordinates {
    /// <p>Latitude</p>
    #[serde(rename = "Latitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// <p>Longitude</p>
    #[serde(rename = "Longitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
}

/// <p>GPS point location dimension</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GPSPointDimension {
    /// <p>Coordinate to measure distance from.</p>
    #[serde(rename = "Coordinates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<GPSCoordinates>,
    /// <p>Range in kilometers from the coordinate.</p>
    #[serde(rename = "RangeInKilometers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_in_kilometers: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAdmChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAdmChannelResponse {
    #[serde(rename = "ADMChannelResponse")]
    pub adm_channel_response: ADMChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApnsChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetApnsChannelResponse {
    #[serde(rename = "APNSChannelResponse")]
    pub apns_channel_response: APNSChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApnsSandboxChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetApnsSandboxChannelResponse {
    #[serde(rename = "APNSSandboxChannelResponse")]
    pub apns_sandbox_channel_response: APNSSandboxChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApnsVoipChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetApnsVoipChannelResponse {
    #[serde(rename = "APNSVoipChannelResponse")]
    pub apns_voip_channel_response: APNSVoipChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApnsVoipSandboxChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetApnsVoipSandboxChannelResponse {
    #[serde(rename = "APNSVoipSandboxChannelResponse")]
    pub apns_voip_sandbox_channel_response: APNSVoipSandboxChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAppRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAppResponse {
    #[serde(rename = "ApplicationResponse")]
    pub application_response: ApplicationResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApplicationSettingsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetApplicationSettingsResponse {
    #[serde(rename = "ApplicationSettingsResource")]
    pub application_settings_resource: ApplicationSettingsResource,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAppsRequest {
    /// <p>The number of entries you want on each page in the response.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAppsResponse {
    #[serde(rename = "ApplicationsResponse")]
    pub applications_response: ApplicationsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBaiduChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetBaiduChannelResponse {
    #[serde(rename = "BaiduChannelResponse")]
    pub baidu_channel_response: BaiduChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCampaignActivitiesRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the campaign.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
    /// <p>The number of entries you want on each page in the response.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCampaignActivitiesResponse {
    #[serde(rename = "ActivitiesResponse")]
    pub activities_response: ActivitiesResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCampaignRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the campaign.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCampaignResponse {
    #[serde(rename = "CampaignResponse")]
    pub campaign_response: CampaignResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCampaignVersionRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the campaign.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
    /// <p>The version of the campaign.</p>
    #[serde(rename = "Version")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCampaignVersionResponse {
    #[serde(rename = "CampaignResponse")]
    pub campaign_response: CampaignResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCampaignVersionsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the campaign.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
    /// <p>The number of entries you want on each page in the response.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCampaignVersionsResponse {
    #[serde(rename = "CampaignsResponse")]
    pub campaigns_response: CampaignsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCampaignsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The number of entries you want on each page in the response.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCampaignsResponse {
    #[serde(rename = "CampaignsResponse")]
    pub campaigns_response: CampaignsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetChannelsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetChannelsResponse {
    #[serde(rename = "ChannelsResponse")]
    pub channels_response: ChannelsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetEmailChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetEmailChannelResponse {
    #[serde(rename = "EmailChannelResponse")]
    pub email_channel_response: EmailChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetEndpointRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the endpoint.</p>
    #[serde(rename = "EndpointId")]
    pub endpoint_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetEndpointResponse {
    #[serde(rename = "EndpointResponse")]
    pub endpoint_response: EndpointResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetEventStreamRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetEventStreamResponse {
    #[serde(rename = "EventStream")]
    pub event_stream: EventStream,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetExportJobRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the job.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetExportJobResponse {
    #[serde(rename = "ExportJobResponse")]
    pub export_job_response: ExportJobResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetExportJobsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The number of entries you want on each page in the response.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetExportJobsResponse {
    #[serde(rename = "ExportJobsResponse")]
    pub export_jobs_response: ExportJobsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGcmChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetGcmChannelResponse {
    #[serde(rename = "GCMChannelResponse")]
    pub gcm_channel_response: GCMChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetImportJobRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the job.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetImportJobResponse {
    #[serde(rename = "ImportJobResponse")]
    pub import_job_response: ImportJobResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetImportJobsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The number of entries you want on each page in the response.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetImportJobsResponse {
    #[serde(rename = "ImportJobsResponse")]
    pub import_jobs_response: ImportJobsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSegmentExportJobsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The number of entries you want on each page in the response.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The unique ID of the segment.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
    /// <p>The NextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSegmentExportJobsResponse {
    #[serde(rename = "ExportJobsResponse")]
    pub export_jobs_response: ExportJobsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSegmentImportJobsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The number of entries you want on each page in the response.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The unique ID of the segment.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
    /// <p>The NextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSegmentImportJobsResponse {
    #[serde(rename = "ImportJobsResponse")]
    pub import_jobs_response: ImportJobsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSegmentRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the segment.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSegmentResponse {
    #[serde(rename = "SegmentResponse")]
    pub segment_response: SegmentResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSegmentVersionRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the segment.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
    /// <p>The segment version.</p>
    #[serde(rename = "Version")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSegmentVersionResponse {
    #[serde(rename = "SegmentResponse")]
    pub segment_response: SegmentResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSegmentVersionsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The number of entries you want on each page in the response.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The unique ID of the segment.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
    /// <p>The NextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSegmentVersionsResponse {
    #[serde(rename = "SegmentsResponse")]
    pub segments_response: SegmentsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSegmentsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The number of entries you want on each page in the response.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSegmentsResponse {
    #[serde(rename = "SegmentsResponse")]
    pub segments_response: SegmentsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSmsChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSmsChannelResponse {
    #[serde(rename = "SMSChannelResponse")]
    pub sms_channel_response: SMSChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUserEndpointsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetUserEndpointsResponse {
    #[serde(rename = "EndpointsResponse")]
    pub endpoints_response: EndpointsResponse,
}

/// <p>Import job request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportJobRequest {
    /// <p>Sets whether the endpoints create a segment when they are imported.</p>
    #[serde(rename = "DefineSegment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub define_segment: Option<bool>,
    /// <p>DEPRECATED. Your AWS account ID, which you assigned to the ExternalID key in an IAM trust policy. Used by Amazon Pinpoint to assume an IAM role. This requirement is removed, and external IDs are not recommended for IAM roles assumed by Amazon Pinpoint.</p>
    #[serde(rename = "ExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The format of the files that contain the endpoint definitions.
    /// Valid values: CSV, JSON</p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>Sets whether the endpoints are registered with Amazon Pinpoint when they are imported.</p>
    #[serde(rename = "RegisterEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_endpoints: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of an IAM role that grants Amazon Pinpoint access to the Amazon S3 location that contains the endpoints to import.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>A URL that points to the location within an Amazon S3 bucket that contains the endpoints to import. The location can be a folder or a single file.
    /// The URL should follow this format: s3://bucket-name/folder-name/file-name</p>
    ///
    /// <p>Amazon Pinpoint will import endpoints from this location and any subfolders it contains.</p>
    #[serde(rename = "S3Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_url: Option<String>,
    /// <p>The ID of the segment to update if the import job is meant to update an existing segment.</p>
    #[serde(rename = "SegmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    /// <p>A custom name for the segment created by the import job. Use if DefineSegment is true.</p>
    #[serde(rename = "SegmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_name: Option<String>,
}

/// <p>Import job resource</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ImportJobResource {
    /// <p>Sets whether the endpoints create a segment when they are imported.</p>
    #[serde(rename = "DefineSegment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub define_segment: Option<bool>,
    /// <p>DEPRECATED. Your AWS account ID, which you assigned to the ExternalID key in an IAM trust policy. Used by Amazon Pinpoint to assume an IAM role. This requirement is removed, and external IDs are not recommended for IAM roles assumed by Amazon Pinpoint.</p>
    #[serde(rename = "ExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The format of the files that contain the endpoint definitions.
    /// Valid values: CSV, JSON</p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>Sets whether the endpoints are registered with Amazon Pinpoint when they are imported.</p>
    #[serde(rename = "RegisterEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_endpoints: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of an IAM role that grants Amazon Pinpoint access to the Amazon S3 location that contains the endpoints to import.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>A URL that points to the location within an Amazon S3 bucket that contains the endpoints to import. The location can be a folder or a single file.
    /// The URL should follow this format: s3://bucket-name/folder-name/file-name</p>
    ///
    /// <p>Amazon Pinpoint will import endpoints from this location and any subfolders it contains.</p>
    #[serde(rename = "S3Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_url: Option<String>,
    /// <p>The ID of the segment to update if the import job is meant to update an existing segment.</p>
    #[serde(rename = "SegmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    /// <p>A custom name for the segment created by the import job. Use if DefineSegment is true.</p>
    #[serde(rename = "SegmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_name: Option<String>,
}

/// <p>Import job response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ImportJobResponse {
    /// <p>The unique ID of the application to which the import job applies.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The number of pieces that have successfully imported as of the time of the request.</p>
    #[serde(rename = "CompletedPieces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_pieces: Option<i64>,
    /// <p>The date the import job completed in ISO 8601 format.</p>
    #[serde(rename = "CompletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<String>,
    /// <p>The date the import job was created in ISO 8601 format.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The import job settings.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<ImportJobResource>,
    /// <p>The number of pieces that have failed to import as of the time of the request.</p>
    #[serde(rename = "FailedPieces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_pieces: Option<i64>,
    /// <p>Provides up to 100 of the first failed entries for the job, if any exist.</p>
    #[serde(rename = "Failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<String>>,
    /// <p>The unique ID of the import job.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The status of the import job.
    /// Valid values: CREATED, INITIALIZING, PROCESSING, COMPLETING, COMPLETED, FAILING, FAILED</p>
    ///
    /// <p>The job status is FAILED if one or more pieces failed to import.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>The number of endpoints that failed to import; for example, because of syntax errors.</p>
    #[serde(rename = "TotalFailures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_failures: Option<i64>,
    /// <p>The total number of pieces that must be imported to finish the job. Each piece is an approximately equal portion of the endpoints to import.</p>
    #[serde(rename = "TotalPieces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pieces: Option<i64>,
    /// <p>The number of endpoints that were processed by the import job.</p>
    #[serde(rename = "TotalProcessed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_processed: Option<i64>,
    /// <p>The job type. Will be Import.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Import job list.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ImportJobsResponse {
    /// <p>A list of import jobs for the application.</p>
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<ImportJobResponse>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Message to send</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// <p>The action that occurs if the user taps a push notification delivered by the campaign:
    /// OPEN_APP - Your app launches, or it becomes the foreground app if it has been sent to the background. This is the default action.</p>
    ///
    /// <p>DEEP_LINK - Uses deep linking features in iOS and Android to open your app and display a designated user interface within the app.</p>
    ///
    /// <p>URL - The default mobile browser on the user&#39;s device launches and opens a web page at the URL you specify.</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The message body. Can include up to 140 characters.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The URL that points to the icon image for the push notification icon, for example, the app icon.</p>
    #[serde(rename = "ImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_icon_url: Option<String>,
    /// <p>The URL that points to the small icon image for the push notification icon, for example, the app icon.</p>
    #[serde(rename = "ImageSmallIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_small_icon_url: Option<String>,
    /// <p>The URL that points to an image used in the push notification.</p>
    #[serde(rename = "ImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// <p>The JSON payload used for a silent push.</p>
    #[serde(rename = "JsonBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_body: Option<String>,
    /// <p>The URL that points to the media resource, for example a .mp4 or .gif file.</p>
    #[serde(rename = "MediaUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    /// <p>The Raw JSON formatted string to be used as the payload. This value overrides the message.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>Indicates if the message should display on the users device.</p>
    ///
    /// <p>Silent pushes can be used for Remote Configuration and Phone Home use cases. </p>
    #[serde(rename = "SilentPush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    /// <p>This parameter specifies how long (in seconds) the message should be kept if the service is unable to deliver the notification the first time. If the value is 0, it treats the notification as if it expires immediately and does not store the notification or attempt to redeliver it. This value is converted to the expiration field when sent to the service. It only applies to APNs and GCM</p>
    #[serde(rename = "TimeToLive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<i64>,
    /// <p>The message title that displays above the message on the user&#39;s device.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The URL to open in the user&#39;s mobile browser. Used if the value for Action is URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Simple message object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct MessageBody {
    /// <p>The error message that&#39;s returned from the API.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The unique message body ID.</p>
    #[serde(rename = "RequestID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// <p>Message configuration for a campaign.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageConfiguration {
    /// <p>The message that the campaign delivers to ADM channels. Overrides the default message.</p>
    #[serde(rename = "ADMMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adm_message: Option<Message>,
    /// <p>The message that the campaign delivers to APNS channels. Overrides the default message.</p>
    #[serde(rename = "APNSMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apns_message: Option<Message>,
    /// <p>The message that the campaign delivers to Baidu channels. Overrides the default message.</p>
    #[serde(rename = "BaiduMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baidu_message: Option<Message>,
    /// <p>The default message for all channels.</p>
    #[serde(rename = "DefaultMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_message: Option<Message>,
    /// <p>The email message configuration.</p>
    #[serde(rename = "EmailMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message: Option<CampaignEmailMessage>,
    /// <p>The message that the campaign delivers to GCM channels. Overrides the default message.</p>
    #[serde(rename = "GCMMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcm_message: Option<Message>,
    /// <p>The SMS message configuration.</p>
    #[serde(rename = "SMSMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_message: Option<CampaignSmsMessage>,
}

/// <p>Send message request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct MessageRequest {
    /// <p>A map of key-value pairs, where each key is an address and each value is an AddressConfiguration object. An address can be a push notification token, a phone number, or an email address.</p>
    #[serde(rename = "Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<::std::collections::HashMap<String, AddressConfiguration>>,
    /// <p>A map of custom attributes to attributes to be attached to the message. This payload is added to the push notification&#39;s &#39;data.pinpoint&#39; object or added to the email/sms delivery receipt event attributes.</p>
    #[serde(rename = "Context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A map of key-value pairs, where each key is an endpoint ID and each value is an EndpointSendConfiguration object. Within an EndpointSendConfiguration object, you can tailor the message for an endpoint by specifying message overrides or substitutions.</p>
    #[serde(rename = "Endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<::std::collections::HashMap<String, EndpointSendConfiguration>>,
    /// <p>Message configuration.</p>
    #[serde(rename = "MessageConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_configuration: Option<DirectMessageConfiguration>,
}

/// <p>Send message response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct MessageResponse {
    /// <p>Application id of the message.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>A map containing a multi part response for each address, with the endpointId as the key and the result as the value.</p>
    #[serde(rename = "EndpointResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_result: Option<::std::collections::HashMap<String, EndpointMessageResult>>,
    /// <p>Original request Id for which this message was delivered.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>A map containing a multi part response for each address, with the address as the key(Email address, phone number or push token) and the result as the value.</p>
    #[serde(rename = "Result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<::std::collections::HashMap<String, MessageResult>>,
}

/// <p>The result from sending a message to an address.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct MessageResult {
    /// <p>Delivery status of message.</p>
    #[serde(rename = "DeliveryStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<String>,
    /// <p>Unique message identifier associated with the message that was sent.</p>
    #[serde(rename = "MessageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// <p>Downstream service status code.</p>
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
    /// <p>Status message for message delivery.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>If token was updated as part of delivery. (This is GCM Specific)</p>
    #[serde(rename = "UpdatedToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_token: Option<String>,
}

/// <p>Custom metric dimension</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricDimension {
    /// <p>GREATER<em>THAN | LESS</em>THAN | GREATER<em>THAN</em>OR<em>EQUAL | LESS</em>THAN<em>OR</em>EQUAL | EQUAL</p>
    #[serde(rename = "ComparisonOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    /// <p>Value to be compared.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// <p>Phone Number Information request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct NumberValidateRequest {
    /// <p>(Optional) The two-character ISO country code for the country where the phone number was originally registered.</p>
    #[serde(rename = "IsoCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_country_code: Option<String>,
    /// <p>The phone number to get information about.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

/// <p>Phone Number Information response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct NumberValidateResponse {
    /// <p>The carrier that the phone number is registered with.</p>
    #[serde(rename = "Carrier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    /// <p>The city where the phone number was originally registered.</p>
    #[serde(rename = "City")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// <p>The cleansed (standardized) phone number in E.164 format.</p>
    #[serde(rename = "CleansedPhoneNumberE164")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleansed_phone_number_e164: Option<String>,
    /// <p>The cleansed phone number in national format.</p>
    #[serde(rename = "CleansedPhoneNumberNational")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleansed_phone_number_national: Option<String>,
    /// <p>The country where the phone number was originally registered.</p>
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// <p>The two-character ISO country code for the country where the phone number was originally registered.</p>
    #[serde(rename = "CountryCodeIso2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code_iso_2: Option<String>,
    /// <p>The numeric country code for the country where the phone number was originally registered.</p>
    #[serde(rename = "CountryCodeNumeric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code_numeric: Option<String>,
    /// <p>The county where the phone number was originally registered.</p>
    #[serde(rename = "County")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub county: Option<String>,
    /// <p>The two-character ISO country code that was included in the request body.</p>
    #[serde(rename = "OriginalCountryCodeIso2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_country_code_iso_2: Option<String>,
    /// <p>The phone number that you included in the request body.</p>
    #[serde(rename = "OriginalPhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_phone_number: Option<String>,
    /// <p>A description of the phone type. Possible values include MOBILE, LANDLINE, VOIP, INVALID, and OTHER.</p>
    #[serde(rename = "PhoneType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_type: Option<String>,
    /// <p>The phone type as an integer. Possible values include 0 (MOBILE), 1 (LANDLINE), 2 (VOIP), 3 (INVALID), and 4 (OTHER).</p>
    #[serde(rename = "PhoneTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_type_code: Option<i64>,
    /// <p>The time zone for the location where the phone number was originally registered.</p>
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// <p>The zip code for the location where the phone number was originally registered.</p>
    #[serde(rename = "ZipCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PhoneNumberValidateRequest {
    #[serde(rename = "NumberValidateRequest")]
    pub number_validate_request: NumberValidateRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PhoneNumberValidateResponse {
    #[serde(rename = "NumberValidateResponse")]
    pub number_validate_response: NumberValidateResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutEventStreamRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "WriteEventStream")]
    pub write_event_stream: WriteEventStream,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutEventStreamResponse {
    #[serde(rename = "EventStream")]
    pub event_stream: EventStream,
}

/// <p>Quiet Time</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuietTime {
    /// <p>The default end time for quiet time in ISO 8601 format.</p>
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// <p>The default start time for quiet time in ISO 8601 format.</p>
    #[serde(rename = "Start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
}

/// <p>Define how a segment based on recency of use.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecencyDimension {
    /// <p>The length of time during which users have been active or inactive with your app.
    /// Valid values: HR<em>24, DAY</em>7, DAY<em>14, DAY</em>30</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// <p>The recency dimension type:
    /// ACTIVE - Users who have used your app within the specified duration are included in the segment.
    /// INACTIVE - Users who have not used your app within the specified duration are included in the segment.</p>
    #[serde(rename = "RecencyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recency_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemoveAttributesRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>Type of attribute. Can be endpoint-custom-attributes, endpoint-custom-metrics, endpoint-user-attributes.</p>
    #[serde(rename = "AttributeType")]
    pub attribute_type: String,
    #[serde(rename = "UpdateAttributesRequest")]
    pub update_attributes_request: UpdateAttributesRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RemoveAttributesResponse {
    #[serde(rename = "AttributesResource")]
    pub attributes_resource: AttributesResource,
}

/// <p>SMS Channel Request</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SMSChannelRequest {
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Sender identifier of your messages.</p>
    #[serde(rename = "SenderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    /// <p>ShortCode registered with phone provider.</p>
    #[serde(rename = "ShortCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_code: Option<String>,
}

/// <p>SMS Channel Response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SMSChannelResponse {
    /// <p>The unique ID of the application to which the SMS channel belongs.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date that the settings were last updated in ISO 8601 format.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Not used. Retained for backwards compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>Channel ID. Not used, only for backwards compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Is this channel archived</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>Who last updated this entry</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>Last date this was updated</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>Platform type. Will be &quot;SMS&quot;</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Promotional messages per second that can be sent</p>
    #[serde(rename = "PromotionalMessagesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotional_messages_per_second: Option<i64>,
    /// <p>Sender identifier of your messages.</p>
    #[serde(rename = "SenderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    /// <p>The short code registered with the phone provider.</p>
    #[serde(rename = "ShortCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_code: Option<String>,
    /// <p>Transactional messages per second that can be sent</p>
    #[serde(rename = "TransactionalMessagesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactional_messages_per_second: Option<i64>,
    /// <p>Version of channel</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>SMS Message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SMSMessage {
    /// <p>The body of the SMS message.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The SMS program name that you provided to AWS Support when you requested your dedicated number.</p>
    #[serde(rename = "Keyword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    /// <p>Is this a transaction priority message or lower priority.</p>
    #[serde(rename = "MessageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// <p>The phone number that the SMS message originates from. Specify one of the dedicated long codes or short codes that you requested from AWS Support and that is assigned to your account. If this attribute is not specified, Amazon Pinpoint randomly assigns a long code.</p>
    #[serde(rename = "OriginationNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination_number: Option<String>,
    /// <p>The sender ID that is shown as the message sender on the recipient&#39;s device. Support for sender IDs varies by country or region.</p>
    #[serde(rename = "SenderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    /// <p>Default message substitutions. Can be overridden by individual address substitutions.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
}

/// <p>Shcedule that defines when a campaign is run.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    /// <p>The scheduled time that the campaign ends in ISO 8601 format.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// <p>How often the campaign delivers messages.</p>
    ///
    /// <p>Valid values: ONCE, HOURLY, DAILY, WEEKLY, MONTHLY</p>
    #[serde(rename = "Frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// <p>Indicates whether the campaign schedule takes effect according to each user&#39;s local time.</p>
    #[serde(rename = "IsLocalTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_local_time: Option<bool>,
    /// <p>The time during which the campaign sends no messages.</p>
    #[serde(rename = "QuietTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_time: Option<QuietTime>,
    /// <p>The scheduled time that the campaign begins in ISO 8601 format.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// <p>The starting UTC offset for the schedule if the value for isLocalTime is true</p>
    ///
    /// <p>Valid values:
    /// UTC
    /// UTC+01
    /// UTC+02
    /// UTC+03
    /// UTC+03:30
    /// UTC+04
    /// UTC+04:30
    /// UTC+05
    /// UTC+05:30
    /// UTC+05:45
    /// UTC+06
    /// UTC+06:30
    /// UTC+07
    /// UTC+08
    /// UTC+09
    /// UTC+09:30
    /// UTC+10
    /// UTC+10:30
    /// UTC+11
    /// UTC+12
    /// UTC+13
    /// UTC-02
    /// UTC-03
    /// UTC-04
    /// UTC-05
    /// UTC-06
    /// UTC-07
    /// UTC-08
    /// UTC-09
    /// UTC-10
    /// UTC-11</p>
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// <p>Segment behavior dimensions</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentBehaviors {
    /// <p>The recency of use.</p>
    #[serde(rename = "Recency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recency: Option<RecencyDimension>,
}

/// <p>Segment demographic dimensions</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentDemographics {
    /// <p>The app version criteria for the segment.</p>
    #[serde(rename = "AppVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<SetDimension>,
    /// <p>The channel criteria for the segment.</p>
    #[serde(rename = "Channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<SetDimension>,
    /// <p>The device type criteria for the segment.</p>
    #[serde(rename = "DeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<SetDimension>,
    /// <p>The device make criteria for the segment.</p>
    #[serde(rename = "Make")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<SetDimension>,
    /// <p>The device model criteria for the segment.</p>
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<SetDimension>,
    /// <p>The device platform criteria for the segment.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<SetDimension>,
}

/// <p>Segment dimensions</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentDimensions {
    /// <p>Custom segment attributes.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, AttributeDimension>>,
    /// <p>The segment behaviors attributes.</p>
    #[serde(rename = "Behavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<SegmentBehaviors>,
    /// <p>The segment demographics attributes.</p>
    #[serde(rename = "Demographic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographic: Option<SegmentDemographics>,
    /// <p>The segment location attributes.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<SegmentLocation>,
    /// <p>Custom segment metrics.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, MetricDimension>>,
    /// <p>Custom segment user attributes.</p>
    #[serde(rename = "UserAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attributes: Option<::std::collections::HashMap<String, AttributeDimension>>,
}

/// <p>Segment group definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentGroup {
    /// <p>List of dimensions to include or exclude.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<SegmentDimensions>>,
    /// <p>Segments that define the source of this segment. Currently a maximum of 1 import segment is supported.</p>
    #[serde(rename = "SourceSegments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_segments: Option<Vec<SegmentReference>>,
    /// <p>Include or exclude the source.</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// <p>How should the dimensions be applied for the result</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Segment group definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentGroupList {
    /// <p>List of dimension groups to evaluate.</p>
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<SegmentGroup>>,
    /// <p>How should the groups be applied for the result</p>
    #[serde(rename = "Include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
}

/// <p>Segment import definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SegmentImportResource {
    /// <p>Channel type counts</p>
    #[serde(rename = "ChannelCounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_counts: Option<::std::collections::HashMap<String, i64>>,
    /// <p>DEPRECATED. Your AWS account ID, which you assigned to the ExternalID key in an IAM trust policy. Used by Amazon Pinpoint to assume an IAM role. This requirement is removed, and external IDs are not recommended for IAM roles assumed by Amazon Pinpoint.</p>
    #[serde(rename = "ExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The format of the endpoint files that were imported to create this segment.
    /// Valid values: CSV, JSON</p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of an IAM role that grants Amazon Pinpoint access to the endpoints in Amazon S3.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>A URL that points to the Amazon S3 location from which the endpoints for this segment were imported.</p>
    #[serde(rename = "S3Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_url: Option<String>,
    /// <p>The number of endpoints that were successfully imported to create this segment.</p>
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// <p>Segment location dimensions</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentLocation {
    /// <p>The country filter according to ISO 3166-1 Alpha-2 codes.</p>
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<SetDimension>,
    /// <p>The GPS Point dimension.</p>
    #[serde(rename = "GPSPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gps_point: Option<GPSPointDimension>,
}

/// <p>Segment reference.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentReference {
    /// <p>Segment Id.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>If specified contains a specific version of the segment included.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Segment definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SegmentResponse {
    /// <p>The ID of the application to which the segment applies.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date the segment was created in ISO 8601 format.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The segment dimensions attributes.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<SegmentDimensions>,
    /// <p>The unique segment ID.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The import job settings.</p>
    #[serde(rename = "ImportDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_definition: Option<SegmentImportResource>,
    /// <p>The date the segment was last updated in ISO 8601 format.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The name of segment</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Segment definition groups. We currently only support one. If specified Dimensions must be empty.</p>
    #[serde(rename = "SegmentGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_groups: Option<SegmentGroupList>,
    /// <p>The segment type:
    /// DIMENSIONAL - A dynamic segment built from selection criteria based on endpoint data reported by your app. You create this type of segment by using the segment builder in the Amazon Pinpoint console or by making a POST request to the segments resource.
    /// IMPORT - A static segment built from an imported set of endpoint definitions. You create this type of segment by importing a segment in the Amazon Pinpoint console or by making a POST request to the jobs/import resource.</p>
    #[serde(rename = "SegmentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_type: Option<String>,
    /// <p>The segment version number.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Segments in your account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SegmentsResponse {
    /// <p>The list of segments.</p>
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<SegmentResponse>>,
    /// <p>An identifier used to retrieve the next page of results. The token is null if no additional pages exist.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SendMessagesRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "MessageRequest")]
    pub message_request: MessageRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SendMessagesResponse {
    #[serde(rename = "MessageResponse")]
    pub message_response: MessageResponse,
}

/// <p>Send message request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SendUsersMessageRequest {
    /// <p>A map of custom attribute-value pairs. Amazon Pinpoint adds these attributes to the data.pinpoint object in the body of the push notification payload. Amazon Pinpoint also provides these attributes in the events that it generates for users-messages deliveries.</p>
    #[serde(rename = "Context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<::std::collections::HashMap<String, String>>,
    /// <p>Message definitions for the default message and any messages that are tailored for specific channels.</p>
    #[serde(rename = "MessageConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_configuration: Option<DirectMessageConfiguration>,
    /// <p>A map that associates user IDs with EndpointSendConfiguration objects. Within an EndpointSendConfiguration object, you can tailor the message for a user by specifying message overrides or substitutions.</p>
    #[serde(rename = "Users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<::std::collections::HashMap<String, EndpointSendConfiguration>>,
}

/// <p>User send message response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SendUsersMessageResponse {
    /// <p>The unique ID of the Amazon Pinpoint project used to send the message.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The unique ID assigned to the users-messages request.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>An object that shows the endpoints that were messaged for each user. The object provides a list of user IDs. For each user ID, it provides the endpoint IDs that were messaged. For each endpoint ID, it provides an EndpointMessageResult object.</p>
    #[serde(rename = "Result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<
        ::std::collections::HashMap<
            String,
            ::std::collections::HashMap<String, EndpointMessageResult>,
        >,
    >,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SendUsersMessagesRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "SendUsersMessageRequest")]
    pub send_users_message_request: SendUsersMessageRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SendUsersMessagesResponse {
    #[serde(rename = "SendUsersMessageResponse")]
    pub send_users_message_response: SendUsersMessageResponse,
}

/// <p>Dimension specification of a segment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDimension {
    /// <p>The type of dimension:
    /// INCLUSIVE - Endpoints that match the criteria are included in the segment.
    /// EXCLUSIVE - Endpoints that match the criteria are excluded from the segment.</p>
    #[serde(rename = "DimensionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_type: Option<String>,
    /// <p>The criteria values for the segment dimension. Endpoints with matching attribute values are included or excluded from the segment, depending on the setting for Type.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Treatment resource</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TreatmentResource {
    /// <p>The unique treatment ID.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The message configuration settings.</p>
    #[serde(rename = "MessageConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_configuration: Option<MessageConfiguration>,
    /// <p>The campaign schedule.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    /// <p>The allocated percentage of users for this treatment.</p>
    #[serde(rename = "SizePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_percent: Option<i64>,
    /// <p>The treatment status.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<CampaignState>,
    /// <p>A custom description for the treatment.</p>
    #[serde(rename = "TreatmentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_description: Option<String>,
    /// <p>The custom name of a variation of the campaign used for A/B testing.</p>
    #[serde(rename = "TreatmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAdmChannelRequest {
    #[serde(rename = "ADMChannelRequest")]
    pub adm_channel_request: ADMChannelRequest,
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateAdmChannelResponse {
    #[serde(rename = "ADMChannelResponse")]
    pub adm_channel_response: ADMChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateApnsChannelRequest {
    #[serde(rename = "APNSChannelRequest")]
    pub apns_channel_request: APNSChannelRequest,
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateApnsChannelResponse {
    #[serde(rename = "APNSChannelResponse")]
    pub apns_channel_response: APNSChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateApnsSandboxChannelRequest {
    #[serde(rename = "APNSSandboxChannelRequest")]
    pub apns_sandbox_channel_request: APNSSandboxChannelRequest,
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateApnsSandboxChannelResponse {
    #[serde(rename = "APNSSandboxChannelResponse")]
    pub apns_sandbox_channel_response: APNSSandboxChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateApnsVoipChannelRequest {
    #[serde(rename = "APNSVoipChannelRequest")]
    pub apns_voip_channel_request: APNSVoipChannelRequest,
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateApnsVoipChannelResponse {
    #[serde(rename = "APNSVoipChannelResponse")]
    pub apns_voip_channel_response: APNSVoipChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateApnsVoipSandboxChannelRequest {
    #[serde(rename = "APNSVoipSandboxChannelRequest")]
    pub apns_voip_sandbox_channel_request: APNSVoipSandboxChannelRequest,
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateApnsVoipSandboxChannelResponse {
    #[serde(rename = "APNSVoipSandboxChannelResponse")]
    pub apns_voip_sandbox_channel_response: APNSVoipSandboxChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateApplicationSettingsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "WriteApplicationSettingsRequest")]
    pub write_application_settings_request: WriteApplicationSettingsRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateApplicationSettingsResponse {
    #[serde(rename = "ApplicationSettingsResource")]
    pub application_settings_resource: ApplicationSettingsResource,
}

/// <p>Update attributes request</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAttributesRequest {
    /// <p>The GLOB wildcard for removing the attributes in the application</p>
    #[serde(rename = "Blacklist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blacklist: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateBaiduChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "BaiduChannelRequest")]
    pub baidu_channel_request: BaiduChannelRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateBaiduChannelResponse {
    #[serde(rename = "BaiduChannelResponse")]
    pub baidu_channel_response: BaiduChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateCampaignRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the campaign.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
    #[serde(rename = "WriteCampaignRequest")]
    pub write_campaign_request: WriteCampaignRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateCampaignResponse {
    #[serde(rename = "CampaignResponse")]
    pub campaign_response: CampaignResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateEmailChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "EmailChannelRequest")]
    pub email_channel_request: EmailChannelRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateEmailChannelResponse {
    #[serde(rename = "EmailChannelResponse")]
    pub email_channel_response: EmailChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateEndpointRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the endpoint.</p>
    #[serde(rename = "EndpointId")]
    pub endpoint_id: String,
    #[serde(rename = "EndpointRequest")]
    pub endpoint_request: EndpointRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateEndpointResponse {
    #[serde(rename = "MessageBody")]
    pub message_body: MessageBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateEndpointsBatchRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "EndpointBatchRequest")]
    pub endpoint_batch_request: EndpointBatchRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateEndpointsBatchResponse {
    #[serde(rename = "MessageBody")]
    pub message_body: MessageBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateGcmChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "GCMChannelRequest")]
    pub gcm_channel_request: GCMChannelRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateGcmChannelResponse {
    #[serde(rename = "GCMChannelResponse")]
    pub gcm_channel_response: GCMChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateSegmentRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the segment.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
    #[serde(rename = "WriteSegmentRequest")]
    pub write_segment_request: WriteSegmentRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateSegmentResponse {
    #[serde(rename = "SegmentResponse")]
    pub segment_response: SegmentResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateSmsChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "SMSChannelRequest")]
    pub sms_channel_request: SMSChannelRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateSmsChannelResponse {
    #[serde(rename = "SMSChannelResponse")]
    pub sms_channel_response: SMSChannelResponse,
}

/// <p>Creating application setting request</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct WriteApplicationSettingsRequest {
    /// <p>Default campaign hook information.</p>
    #[serde(rename = "CampaignHook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_hook: Option<CampaignHook>,
    /// <p>The CloudWatchMetrics settings for the app.</p>
    #[serde(rename = "CloudWatchMetricsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_metrics_enabled: Option<bool>,
    /// <p>The default campaign limits for the app. These limits apply to each campaign for the app, unless the campaign overrides the default with limits of its own.</p>
    #[serde(rename = "Limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<CampaignLimits>,
    /// <p>The default quiet time for the app. Each campaign for this app sends no messages during this time unless the campaign overrides the default with a quiet time of its own.</p>
    #[serde(rename = "QuietTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_time: Option<QuietTime>,
}

/// <p>Used to create a campaign.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct WriteCampaignRequest {
    /// <p>Treatments that are defined in addition to the default treatment.</p>
    #[serde(rename = "AdditionalTreatments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_treatments: Option<Vec<WriteTreatmentResource>>,
    /// <p>A description of the campaign.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The allocated percentage of end users who will not receive messages from this campaign.</p>
    #[serde(rename = "HoldoutPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holdout_percent: Option<i64>,
    /// <p>Campaign hook information.</p>
    #[serde(rename = "Hook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook: Option<CampaignHook>,
    /// <p>Indicates whether the campaign is paused. A paused campaign does not send messages unless you resume it by setting IsPaused to false.</p>
    #[serde(rename = "IsPaused")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_paused: Option<bool>,
    /// <p>The campaign limits settings.</p>
    #[serde(rename = "Limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<CampaignLimits>,
    /// <p>The message configuration settings.</p>
    #[serde(rename = "MessageConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_configuration: Option<MessageConfiguration>,
    /// <p>The custom name of the campaign.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The campaign schedule.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    /// <p>The ID of the segment to which the campaign sends messages.</p>
    #[serde(rename = "SegmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    /// <p>The version of the segment to which the campaign sends messages.</p>
    #[serde(rename = "SegmentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_version: Option<i64>,
    /// <p>A custom description for the treatment.</p>
    #[serde(rename = "TreatmentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_description: Option<String>,
    /// <p>The custom name of a variation of the campaign used for A/B testing.</p>
    #[serde(rename = "TreatmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_name: Option<String>,
}

/// <p>Request to save an EventStream.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct WriteEventStream {
    /// <p>The Amazon Resource Name (ARN) of the Amazon Kinesis stream or Firehose delivery stream to which you want to publish events.
    /// Firehose ARN: arn:aws:firehose:REGION:ACCOUNT<em>ID:deliverystream/STREAM</em>NAME
    /// Kinesis ARN: arn:aws:kinesis:REGION:ACCOUNT<em>ID:stream/STREAM</em>NAME</p>
    #[serde(rename = "DestinationStreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_stream_arn: Option<String>,
    /// <p>The IAM role that authorizes Amazon Pinpoint to publish events to the stream in your account.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>Segment definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct WriteSegmentRequest {
    /// <p>The segment dimensions attributes.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<SegmentDimensions>,
    /// <p>The name of segment</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Segment definition groups. We currently only support one. If specified Dimensions must be empty.</p>
    #[serde(rename = "SegmentGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_groups: Option<SegmentGroupList>,
}

/// <p>Used to create a campaign treatment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct WriteTreatmentResource {
    /// <p>The message configuration settings.</p>
    #[serde(rename = "MessageConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_configuration: Option<MessageConfiguration>,
    /// <p>The campaign schedule.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    /// <p>The allocated percentage of users for this treatment.</p>
    #[serde(rename = "SizePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_percent: Option<i64>,
    /// <p>A custom description for the treatment.</p>
    #[serde(rename = "TreatmentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_description: Option<String>,
    /// <p>The custom name of a variation of the campaign used for A/B testing.</p>
    #[serde(rename = "TreatmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_name: Option<String>,
}

/// Errors returned by CreateApp
#[derive(Debug, PartialEq)]
pub enum CreateAppError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateAppError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateAppError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return CreateAppError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return CreateAppError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return CreateAppError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return CreateAppError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => return CreateAppError::NotFound(String::from(error_message)),
                "TooManyRequestsException" => {
                    return CreateAppError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateAppError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateAppError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateAppError {
    fn from(err: serde_json::error::Error) -> CreateAppError {
        CreateAppError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateAppError {
    fn from(err: CredentialsError) -> CreateAppError {
        CreateAppError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAppError {
    fn from(err: HttpDispatchError) -> CreateAppError {
        CreateAppError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateAppError {
    fn from(err: io::Error) -> CreateAppError {
        CreateAppError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAppError {
    fn description(&self) -> &str {
        match *self {
            CreateAppError::BadRequest(ref cause) => cause,
            CreateAppError::Forbidden(ref cause) => cause,
            CreateAppError::InternalServerError(ref cause) => cause,
            CreateAppError::MethodNotAllowed(ref cause) => cause,
            CreateAppError::NotFound(ref cause) => cause,
            CreateAppError::TooManyRequests(ref cause) => cause,
            CreateAppError::Validation(ref cause) => cause,
            CreateAppError::Credentials(ref err) => err.description(),
            CreateAppError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateAppError::ParseError(ref cause) => cause,
            CreateAppError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateCampaign
#[derive(Debug, PartialEq)]
pub enum CreateCampaignError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateCampaignError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateCampaignError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return CreateCampaignError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return CreateCampaignError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return CreateCampaignError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return CreateCampaignError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return CreateCampaignError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return CreateCampaignError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateCampaignError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateCampaignError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateCampaignError {
    fn from(err: serde_json::error::Error) -> CreateCampaignError {
        CreateCampaignError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateCampaignError {
    fn from(err: CredentialsError) -> CreateCampaignError {
        CreateCampaignError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCampaignError {
    fn from(err: HttpDispatchError) -> CreateCampaignError {
        CreateCampaignError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateCampaignError {
    fn from(err: io::Error) -> CreateCampaignError {
        CreateCampaignError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateCampaignError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCampaignError {
    fn description(&self) -> &str {
        match *self {
            CreateCampaignError::BadRequest(ref cause) => cause,
            CreateCampaignError::Forbidden(ref cause) => cause,
            CreateCampaignError::InternalServerError(ref cause) => cause,
            CreateCampaignError::MethodNotAllowed(ref cause) => cause,
            CreateCampaignError::NotFound(ref cause) => cause,
            CreateCampaignError::TooManyRequests(ref cause) => cause,
            CreateCampaignError::Validation(ref cause) => cause,
            CreateCampaignError::Credentials(ref err) => err.description(),
            CreateCampaignError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateCampaignError::ParseError(ref cause) => cause,
            CreateCampaignError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateExportJob
#[derive(Debug, PartialEq)]
pub enum CreateExportJobError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateExportJobError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateExportJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return CreateExportJobError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return CreateExportJobError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return CreateExportJobError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return CreateExportJobError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return CreateExportJobError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return CreateExportJobError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateExportJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateExportJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateExportJobError {
    fn from(err: serde_json::error::Error) -> CreateExportJobError {
        CreateExportJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateExportJobError {
    fn from(err: CredentialsError) -> CreateExportJobError {
        CreateExportJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateExportJobError {
    fn from(err: HttpDispatchError) -> CreateExportJobError {
        CreateExportJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateExportJobError {
    fn from(err: io::Error) -> CreateExportJobError {
        CreateExportJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateExportJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateExportJobError {
    fn description(&self) -> &str {
        match *self {
            CreateExportJobError::BadRequest(ref cause) => cause,
            CreateExportJobError::Forbidden(ref cause) => cause,
            CreateExportJobError::InternalServerError(ref cause) => cause,
            CreateExportJobError::MethodNotAllowed(ref cause) => cause,
            CreateExportJobError::NotFound(ref cause) => cause,
            CreateExportJobError::TooManyRequests(ref cause) => cause,
            CreateExportJobError::Validation(ref cause) => cause,
            CreateExportJobError::Credentials(ref err) => err.description(),
            CreateExportJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateExportJobError::ParseError(ref cause) => cause,
            CreateExportJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateImportJob
#[derive(Debug, PartialEq)]
pub enum CreateImportJobError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateImportJobError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateImportJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return CreateImportJobError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return CreateImportJobError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return CreateImportJobError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return CreateImportJobError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return CreateImportJobError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return CreateImportJobError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateImportJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateImportJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateImportJobError {
    fn from(err: serde_json::error::Error) -> CreateImportJobError {
        CreateImportJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateImportJobError {
    fn from(err: CredentialsError) -> CreateImportJobError {
        CreateImportJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateImportJobError {
    fn from(err: HttpDispatchError) -> CreateImportJobError {
        CreateImportJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateImportJobError {
    fn from(err: io::Error) -> CreateImportJobError {
        CreateImportJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateImportJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateImportJobError {
    fn description(&self) -> &str {
        match *self {
            CreateImportJobError::BadRequest(ref cause) => cause,
            CreateImportJobError::Forbidden(ref cause) => cause,
            CreateImportJobError::InternalServerError(ref cause) => cause,
            CreateImportJobError::MethodNotAllowed(ref cause) => cause,
            CreateImportJobError::NotFound(ref cause) => cause,
            CreateImportJobError::TooManyRequests(ref cause) => cause,
            CreateImportJobError::Validation(ref cause) => cause,
            CreateImportJobError::Credentials(ref err) => err.description(),
            CreateImportJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateImportJobError::ParseError(ref cause) => cause,
            CreateImportJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateSegment
#[derive(Debug, PartialEq)]
pub enum CreateSegmentError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateSegmentError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateSegmentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return CreateSegmentError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return CreateSegmentError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return CreateSegmentError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return CreateSegmentError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return CreateSegmentError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return CreateSegmentError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateSegmentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateSegmentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateSegmentError {
    fn from(err: serde_json::error::Error) -> CreateSegmentError {
        CreateSegmentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateSegmentError {
    fn from(err: CredentialsError) -> CreateSegmentError {
        CreateSegmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSegmentError {
    fn from(err: HttpDispatchError) -> CreateSegmentError {
        CreateSegmentError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateSegmentError {
    fn from(err: io::Error) -> CreateSegmentError {
        CreateSegmentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateSegmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSegmentError {
    fn description(&self) -> &str {
        match *self {
            CreateSegmentError::BadRequest(ref cause) => cause,
            CreateSegmentError::Forbidden(ref cause) => cause,
            CreateSegmentError::InternalServerError(ref cause) => cause,
            CreateSegmentError::MethodNotAllowed(ref cause) => cause,
            CreateSegmentError::NotFound(ref cause) => cause,
            CreateSegmentError::TooManyRequests(ref cause) => cause,
            CreateSegmentError::Validation(ref cause) => cause,
            CreateSegmentError::Credentials(ref err) => err.description(),
            CreateSegmentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateSegmentError::ParseError(ref cause) => cause,
            CreateSegmentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteAdmChannel
#[derive(Debug, PartialEq)]
pub enum DeleteAdmChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteAdmChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteAdmChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteAdmChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return DeleteAdmChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DeleteAdmChannelError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return DeleteAdmChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteAdmChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DeleteAdmChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteAdmChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteAdmChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteAdmChannelError {
    fn from(err: serde_json::error::Error) -> DeleteAdmChannelError {
        DeleteAdmChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteAdmChannelError {
    fn from(err: CredentialsError) -> DeleteAdmChannelError {
        DeleteAdmChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAdmChannelError {
    fn from(err: HttpDispatchError) -> DeleteAdmChannelError {
        DeleteAdmChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAdmChannelError {
    fn from(err: io::Error) -> DeleteAdmChannelError {
        DeleteAdmChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAdmChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAdmChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteAdmChannelError::BadRequest(ref cause) => cause,
            DeleteAdmChannelError::Forbidden(ref cause) => cause,
            DeleteAdmChannelError::InternalServerError(ref cause) => cause,
            DeleteAdmChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteAdmChannelError::NotFound(ref cause) => cause,
            DeleteAdmChannelError::TooManyRequests(ref cause) => cause,
            DeleteAdmChannelError::Validation(ref cause) => cause,
            DeleteAdmChannelError::Credentials(ref err) => err.description(),
            DeleteAdmChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteAdmChannelError::ParseError(ref cause) => cause,
            DeleteAdmChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteApnsChannel
#[derive(Debug, PartialEq)]
pub enum DeleteApnsChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteApnsChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteApnsChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteApnsChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return DeleteApnsChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DeleteApnsChannelError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return DeleteApnsChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteApnsChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DeleteApnsChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteApnsChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteApnsChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteApnsChannelError {
    fn from(err: serde_json::error::Error) -> DeleteApnsChannelError {
        DeleteApnsChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteApnsChannelError {
    fn from(err: CredentialsError) -> DeleteApnsChannelError {
        DeleteApnsChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteApnsChannelError {
    fn from(err: HttpDispatchError) -> DeleteApnsChannelError {
        DeleteApnsChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteApnsChannelError {
    fn from(err: io::Error) -> DeleteApnsChannelError {
        DeleteApnsChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteApnsChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApnsChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteApnsChannelError::BadRequest(ref cause) => cause,
            DeleteApnsChannelError::Forbidden(ref cause) => cause,
            DeleteApnsChannelError::InternalServerError(ref cause) => cause,
            DeleteApnsChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteApnsChannelError::NotFound(ref cause) => cause,
            DeleteApnsChannelError::TooManyRequests(ref cause) => cause,
            DeleteApnsChannelError::Validation(ref cause) => cause,
            DeleteApnsChannelError::Credentials(ref err) => err.description(),
            DeleteApnsChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteApnsChannelError::ParseError(ref cause) => cause,
            DeleteApnsChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteApnsSandboxChannel
#[derive(Debug, PartialEq)]
pub enum DeleteApnsSandboxChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteApnsSandboxChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteApnsSandboxChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteApnsSandboxChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return DeleteApnsSandboxChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DeleteApnsSandboxChannelError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "MethodNotAllowedException" => {
                    return DeleteApnsSandboxChannelError::MethodNotAllowed(String::from(
                        error_message,
                    ))
                }
                "NotFoundException" => {
                    return DeleteApnsSandboxChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DeleteApnsSandboxChannelError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DeleteApnsSandboxChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteApnsSandboxChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteApnsSandboxChannelError {
    fn from(err: serde_json::error::Error) -> DeleteApnsSandboxChannelError {
        DeleteApnsSandboxChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteApnsSandboxChannelError {
    fn from(err: CredentialsError) -> DeleteApnsSandboxChannelError {
        DeleteApnsSandboxChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteApnsSandboxChannelError {
    fn from(err: HttpDispatchError) -> DeleteApnsSandboxChannelError {
        DeleteApnsSandboxChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteApnsSandboxChannelError {
    fn from(err: io::Error) -> DeleteApnsSandboxChannelError {
        DeleteApnsSandboxChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteApnsSandboxChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApnsSandboxChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteApnsSandboxChannelError::BadRequest(ref cause) => cause,
            DeleteApnsSandboxChannelError::Forbidden(ref cause) => cause,
            DeleteApnsSandboxChannelError::InternalServerError(ref cause) => cause,
            DeleteApnsSandboxChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteApnsSandboxChannelError::NotFound(ref cause) => cause,
            DeleteApnsSandboxChannelError::TooManyRequests(ref cause) => cause,
            DeleteApnsSandboxChannelError::Validation(ref cause) => cause,
            DeleteApnsSandboxChannelError::Credentials(ref err) => err.description(),
            DeleteApnsSandboxChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteApnsSandboxChannelError::ParseError(ref cause) => cause,
            DeleteApnsSandboxChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteApnsVoipChannel
#[derive(Debug, PartialEq)]
pub enum DeleteApnsVoipChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteApnsVoipChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteApnsVoipChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteApnsVoipChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return DeleteApnsVoipChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DeleteApnsVoipChannelError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "MethodNotAllowedException" => {
                    return DeleteApnsVoipChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteApnsVoipChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DeleteApnsVoipChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteApnsVoipChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteApnsVoipChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteApnsVoipChannelError {
    fn from(err: serde_json::error::Error) -> DeleteApnsVoipChannelError {
        DeleteApnsVoipChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteApnsVoipChannelError {
    fn from(err: CredentialsError) -> DeleteApnsVoipChannelError {
        DeleteApnsVoipChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteApnsVoipChannelError {
    fn from(err: HttpDispatchError) -> DeleteApnsVoipChannelError {
        DeleteApnsVoipChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteApnsVoipChannelError {
    fn from(err: io::Error) -> DeleteApnsVoipChannelError {
        DeleteApnsVoipChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteApnsVoipChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApnsVoipChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteApnsVoipChannelError::BadRequest(ref cause) => cause,
            DeleteApnsVoipChannelError::Forbidden(ref cause) => cause,
            DeleteApnsVoipChannelError::InternalServerError(ref cause) => cause,
            DeleteApnsVoipChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteApnsVoipChannelError::NotFound(ref cause) => cause,
            DeleteApnsVoipChannelError::TooManyRequests(ref cause) => cause,
            DeleteApnsVoipChannelError::Validation(ref cause) => cause,
            DeleteApnsVoipChannelError::Credentials(ref err) => err.description(),
            DeleteApnsVoipChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteApnsVoipChannelError::ParseError(ref cause) => cause,
            DeleteApnsVoipChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteApnsVoipSandboxChannel
#[derive(Debug, PartialEq)]
pub enum DeleteApnsVoipSandboxChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteApnsVoipSandboxChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteApnsVoipSandboxChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteApnsVoipSandboxChannelError::BadRequest(String::from(
                        error_message,
                    ))
                }
                "ForbiddenException" => {
                    return DeleteApnsVoipSandboxChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DeleteApnsVoipSandboxChannelError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "MethodNotAllowedException" => {
                    return DeleteApnsVoipSandboxChannelError::MethodNotAllowed(String::from(
                        error_message,
                    ))
                }
                "NotFoundException" => {
                    return DeleteApnsVoipSandboxChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DeleteApnsVoipSandboxChannelError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DeleteApnsVoipSandboxChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteApnsVoipSandboxChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteApnsVoipSandboxChannelError {
    fn from(err: serde_json::error::Error) -> DeleteApnsVoipSandboxChannelError {
        DeleteApnsVoipSandboxChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteApnsVoipSandboxChannelError {
    fn from(err: CredentialsError) -> DeleteApnsVoipSandboxChannelError {
        DeleteApnsVoipSandboxChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteApnsVoipSandboxChannelError {
    fn from(err: HttpDispatchError) -> DeleteApnsVoipSandboxChannelError {
        DeleteApnsVoipSandboxChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteApnsVoipSandboxChannelError {
    fn from(err: io::Error) -> DeleteApnsVoipSandboxChannelError {
        DeleteApnsVoipSandboxChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteApnsVoipSandboxChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApnsVoipSandboxChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteApnsVoipSandboxChannelError::BadRequest(ref cause) => cause,
            DeleteApnsVoipSandboxChannelError::Forbidden(ref cause) => cause,
            DeleteApnsVoipSandboxChannelError::InternalServerError(ref cause) => cause,
            DeleteApnsVoipSandboxChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteApnsVoipSandboxChannelError::NotFound(ref cause) => cause,
            DeleteApnsVoipSandboxChannelError::TooManyRequests(ref cause) => cause,
            DeleteApnsVoipSandboxChannelError::Validation(ref cause) => cause,
            DeleteApnsVoipSandboxChannelError::Credentials(ref err) => err.description(),
            DeleteApnsVoipSandboxChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteApnsVoipSandboxChannelError::ParseError(ref cause) => cause,
            DeleteApnsVoipSandboxChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteApp
#[derive(Debug, PartialEq)]
pub enum DeleteAppError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteAppError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteAppError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteAppError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return DeleteAppError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DeleteAppError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return DeleteAppError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => return DeleteAppError::NotFound(String::from(error_message)),
                "TooManyRequestsException" => {
                    return DeleteAppError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteAppError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteAppError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteAppError {
    fn from(err: serde_json::error::Error) -> DeleteAppError {
        DeleteAppError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteAppError {
    fn from(err: CredentialsError) -> DeleteAppError {
        DeleteAppError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAppError {
    fn from(err: HttpDispatchError) -> DeleteAppError {
        DeleteAppError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAppError {
    fn from(err: io::Error) -> DeleteAppError {
        DeleteAppError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAppError {
    fn description(&self) -> &str {
        match *self {
            DeleteAppError::BadRequest(ref cause) => cause,
            DeleteAppError::Forbidden(ref cause) => cause,
            DeleteAppError::InternalServerError(ref cause) => cause,
            DeleteAppError::MethodNotAllowed(ref cause) => cause,
            DeleteAppError::NotFound(ref cause) => cause,
            DeleteAppError::TooManyRequests(ref cause) => cause,
            DeleteAppError::Validation(ref cause) => cause,
            DeleteAppError::Credentials(ref err) => err.description(),
            DeleteAppError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteAppError::ParseError(ref cause) => cause,
            DeleteAppError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteBaiduChannel
#[derive(Debug, PartialEq)]
pub enum DeleteBaiduChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteBaiduChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteBaiduChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteBaiduChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return DeleteBaiduChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DeleteBaiduChannelError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return DeleteBaiduChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteBaiduChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DeleteBaiduChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteBaiduChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteBaiduChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteBaiduChannelError {
    fn from(err: serde_json::error::Error) -> DeleteBaiduChannelError {
        DeleteBaiduChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteBaiduChannelError {
    fn from(err: CredentialsError) -> DeleteBaiduChannelError {
        DeleteBaiduChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBaiduChannelError {
    fn from(err: HttpDispatchError) -> DeleteBaiduChannelError {
        DeleteBaiduChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBaiduChannelError {
    fn from(err: io::Error) -> DeleteBaiduChannelError {
        DeleteBaiduChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBaiduChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBaiduChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteBaiduChannelError::BadRequest(ref cause) => cause,
            DeleteBaiduChannelError::Forbidden(ref cause) => cause,
            DeleteBaiduChannelError::InternalServerError(ref cause) => cause,
            DeleteBaiduChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteBaiduChannelError::NotFound(ref cause) => cause,
            DeleteBaiduChannelError::TooManyRequests(ref cause) => cause,
            DeleteBaiduChannelError::Validation(ref cause) => cause,
            DeleteBaiduChannelError::Credentials(ref err) => err.description(),
            DeleteBaiduChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBaiduChannelError::ParseError(ref cause) => cause,
            DeleteBaiduChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteCampaign
#[derive(Debug, PartialEq)]
pub enum DeleteCampaignError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteCampaignError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteCampaignError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteCampaignError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return DeleteCampaignError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DeleteCampaignError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return DeleteCampaignError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteCampaignError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DeleteCampaignError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteCampaignError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteCampaignError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteCampaignError {
    fn from(err: serde_json::error::Error) -> DeleteCampaignError {
        DeleteCampaignError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteCampaignError {
    fn from(err: CredentialsError) -> DeleteCampaignError {
        DeleteCampaignError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCampaignError {
    fn from(err: HttpDispatchError) -> DeleteCampaignError {
        DeleteCampaignError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteCampaignError {
    fn from(err: io::Error) -> DeleteCampaignError {
        DeleteCampaignError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteCampaignError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCampaignError {
    fn description(&self) -> &str {
        match *self {
            DeleteCampaignError::BadRequest(ref cause) => cause,
            DeleteCampaignError::Forbidden(ref cause) => cause,
            DeleteCampaignError::InternalServerError(ref cause) => cause,
            DeleteCampaignError::MethodNotAllowed(ref cause) => cause,
            DeleteCampaignError::NotFound(ref cause) => cause,
            DeleteCampaignError::TooManyRequests(ref cause) => cause,
            DeleteCampaignError::Validation(ref cause) => cause,
            DeleteCampaignError::Credentials(ref err) => err.description(),
            DeleteCampaignError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteCampaignError::ParseError(ref cause) => cause,
            DeleteCampaignError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteEmailChannel
#[derive(Debug, PartialEq)]
pub enum DeleteEmailChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteEmailChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteEmailChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteEmailChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return DeleteEmailChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DeleteEmailChannelError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return DeleteEmailChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteEmailChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DeleteEmailChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteEmailChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteEmailChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteEmailChannelError {
    fn from(err: serde_json::error::Error) -> DeleteEmailChannelError {
        DeleteEmailChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteEmailChannelError {
    fn from(err: CredentialsError) -> DeleteEmailChannelError {
        DeleteEmailChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteEmailChannelError {
    fn from(err: HttpDispatchError) -> DeleteEmailChannelError {
        DeleteEmailChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteEmailChannelError {
    fn from(err: io::Error) -> DeleteEmailChannelError {
        DeleteEmailChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteEmailChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEmailChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteEmailChannelError::BadRequest(ref cause) => cause,
            DeleteEmailChannelError::Forbidden(ref cause) => cause,
            DeleteEmailChannelError::InternalServerError(ref cause) => cause,
            DeleteEmailChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteEmailChannelError::NotFound(ref cause) => cause,
            DeleteEmailChannelError::TooManyRequests(ref cause) => cause,
            DeleteEmailChannelError::Validation(ref cause) => cause,
            DeleteEmailChannelError::Credentials(ref err) => err.description(),
            DeleteEmailChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteEmailChannelError::ParseError(ref cause) => cause,
            DeleteEmailChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteEndpoint
#[derive(Debug, PartialEq)]
pub enum DeleteEndpointError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteEndpointError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteEndpointError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteEndpointError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return DeleteEndpointError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DeleteEndpointError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return DeleteEndpointError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteEndpointError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DeleteEndpointError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteEndpointError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteEndpointError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteEndpointError {
    fn from(err: serde_json::error::Error) -> DeleteEndpointError {
        DeleteEndpointError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteEndpointError {
    fn from(err: CredentialsError) -> DeleteEndpointError {
        DeleteEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteEndpointError {
    fn from(err: HttpDispatchError) -> DeleteEndpointError {
        DeleteEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteEndpointError {
    fn from(err: io::Error) -> DeleteEndpointError {
        DeleteEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEndpointError {
    fn description(&self) -> &str {
        match *self {
            DeleteEndpointError::BadRequest(ref cause) => cause,
            DeleteEndpointError::Forbidden(ref cause) => cause,
            DeleteEndpointError::InternalServerError(ref cause) => cause,
            DeleteEndpointError::MethodNotAllowed(ref cause) => cause,
            DeleteEndpointError::NotFound(ref cause) => cause,
            DeleteEndpointError::TooManyRequests(ref cause) => cause,
            DeleteEndpointError::Validation(ref cause) => cause,
            DeleteEndpointError::Credentials(ref err) => err.description(),
            DeleteEndpointError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteEndpointError::ParseError(ref cause) => cause,
            DeleteEndpointError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteEventStream
#[derive(Debug, PartialEq)]
pub enum DeleteEventStreamError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteEventStreamError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteEventStreamError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteEventStreamError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return DeleteEventStreamError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DeleteEventStreamError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return DeleteEventStreamError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteEventStreamError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DeleteEventStreamError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteEventStreamError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteEventStreamError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteEventStreamError {
    fn from(err: serde_json::error::Error) -> DeleteEventStreamError {
        DeleteEventStreamError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteEventStreamError {
    fn from(err: CredentialsError) -> DeleteEventStreamError {
        DeleteEventStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteEventStreamError {
    fn from(err: HttpDispatchError) -> DeleteEventStreamError {
        DeleteEventStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteEventStreamError {
    fn from(err: io::Error) -> DeleteEventStreamError {
        DeleteEventStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteEventStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEventStreamError {
    fn description(&self) -> &str {
        match *self {
            DeleteEventStreamError::BadRequest(ref cause) => cause,
            DeleteEventStreamError::Forbidden(ref cause) => cause,
            DeleteEventStreamError::InternalServerError(ref cause) => cause,
            DeleteEventStreamError::MethodNotAllowed(ref cause) => cause,
            DeleteEventStreamError::NotFound(ref cause) => cause,
            DeleteEventStreamError::TooManyRequests(ref cause) => cause,
            DeleteEventStreamError::Validation(ref cause) => cause,
            DeleteEventStreamError::Credentials(ref err) => err.description(),
            DeleteEventStreamError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteEventStreamError::ParseError(ref cause) => cause,
            DeleteEventStreamError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteGcmChannel
#[derive(Debug, PartialEq)]
pub enum DeleteGcmChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteGcmChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteGcmChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteGcmChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return DeleteGcmChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DeleteGcmChannelError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return DeleteGcmChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteGcmChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DeleteGcmChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteGcmChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteGcmChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteGcmChannelError {
    fn from(err: serde_json::error::Error) -> DeleteGcmChannelError {
        DeleteGcmChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteGcmChannelError {
    fn from(err: CredentialsError) -> DeleteGcmChannelError {
        DeleteGcmChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteGcmChannelError {
    fn from(err: HttpDispatchError) -> DeleteGcmChannelError {
        DeleteGcmChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteGcmChannelError {
    fn from(err: io::Error) -> DeleteGcmChannelError {
        DeleteGcmChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteGcmChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteGcmChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteGcmChannelError::BadRequest(ref cause) => cause,
            DeleteGcmChannelError::Forbidden(ref cause) => cause,
            DeleteGcmChannelError::InternalServerError(ref cause) => cause,
            DeleteGcmChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteGcmChannelError::NotFound(ref cause) => cause,
            DeleteGcmChannelError::TooManyRequests(ref cause) => cause,
            DeleteGcmChannelError::Validation(ref cause) => cause,
            DeleteGcmChannelError::Credentials(ref err) => err.description(),
            DeleteGcmChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteGcmChannelError::ParseError(ref cause) => cause,
            DeleteGcmChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteSegment
#[derive(Debug, PartialEq)]
pub enum DeleteSegmentError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteSegmentError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteSegmentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteSegmentError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return DeleteSegmentError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DeleteSegmentError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return DeleteSegmentError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteSegmentError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DeleteSegmentError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteSegmentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteSegmentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteSegmentError {
    fn from(err: serde_json::error::Error) -> DeleteSegmentError {
        DeleteSegmentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSegmentError {
    fn from(err: CredentialsError) -> DeleteSegmentError {
        DeleteSegmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSegmentError {
    fn from(err: HttpDispatchError) -> DeleteSegmentError {
        DeleteSegmentError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSegmentError {
    fn from(err: io::Error) -> DeleteSegmentError {
        DeleteSegmentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSegmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSegmentError {
    fn description(&self) -> &str {
        match *self {
            DeleteSegmentError::BadRequest(ref cause) => cause,
            DeleteSegmentError::Forbidden(ref cause) => cause,
            DeleteSegmentError::InternalServerError(ref cause) => cause,
            DeleteSegmentError::MethodNotAllowed(ref cause) => cause,
            DeleteSegmentError::NotFound(ref cause) => cause,
            DeleteSegmentError::TooManyRequests(ref cause) => cause,
            DeleteSegmentError::Validation(ref cause) => cause,
            DeleteSegmentError::Credentials(ref err) => err.description(),
            DeleteSegmentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteSegmentError::ParseError(ref cause) => cause,
            DeleteSegmentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteSmsChannel
#[derive(Debug, PartialEq)]
pub enum DeleteSmsChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteSmsChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteSmsChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteSmsChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return DeleteSmsChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DeleteSmsChannelError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return DeleteSmsChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteSmsChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DeleteSmsChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteSmsChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteSmsChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteSmsChannelError {
    fn from(err: serde_json::error::Error) -> DeleteSmsChannelError {
        DeleteSmsChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSmsChannelError {
    fn from(err: CredentialsError) -> DeleteSmsChannelError {
        DeleteSmsChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSmsChannelError {
    fn from(err: HttpDispatchError) -> DeleteSmsChannelError {
        DeleteSmsChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSmsChannelError {
    fn from(err: io::Error) -> DeleteSmsChannelError {
        DeleteSmsChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSmsChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSmsChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteSmsChannelError::BadRequest(ref cause) => cause,
            DeleteSmsChannelError::Forbidden(ref cause) => cause,
            DeleteSmsChannelError::InternalServerError(ref cause) => cause,
            DeleteSmsChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteSmsChannelError::NotFound(ref cause) => cause,
            DeleteSmsChannelError::TooManyRequests(ref cause) => cause,
            DeleteSmsChannelError::Validation(ref cause) => cause,
            DeleteSmsChannelError::Credentials(ref err) => err.description(),
            DeleteSmsChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteSmsChannelError::ParseError(ref cause) => cause,
            DeleteSmsChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteUserEndpoints
#[derive(Debug, PartialEq)]
pub enum DeleteUserEndpointsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteUserEndpointsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteUserEndpointsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteUserEndpointsError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return DeleteUserEndpointsError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return DeleteUserEndpointsError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "MethodNotAllowedException" => {
                    return DeleteUserEndpointsError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteUserEndpointsError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DeleteUserEndpointsError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteUserEndpointsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteUserEndpointsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteUserEndpointsError {
    fn from(err: serde_json::error::Error) -> DeleteUserEndpointsError {
        DeleteUserEndpointsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteUserEndpointsError {
    fn from(err: CredentialsError) -> DeleteUserEndpointsError {
        DeleteUserEndpointsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteUserEndpointsError {
    fn from(err: HttpDispatchError) -> DeleteUserEndpointsError {
        DeleteUserEndpointsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteUserEndpointsError {
    fn from(err: io::Error) -> DeleteUserEndpointsError {
        DeleteUserEndpointsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteUserEndpointsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUserEndpointsError {
    fn description(&self) -> &str {
        match *self {
            DeleteUserEndpointsError::BadRequest(ref cause) => cause,
            DeleteUserEndpointsError::Forbidden(ref cause) => cause,
            DeleteUserEndpointsError::InternalServerError(ref cause) => cause,
            DeleteUserEndpointsError::MethodNotAllowed(ref cause) => cause,
            DeleteUserEndpointsError::NotFound(ref cause) => cause,
            DeleteUserEndpointsError::TooManyRequests(ref cause) => cause,
            DeleteUserEndpointsError::Validation(ref cause) => cause,
            DeleteUserEndpointsError::Credentials(ref err) => err.description(),
            DeleteUserEndpointsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteUserEndpointsError::ParseError(ref cause) => cause,
            DeleteUserEndpointsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetAdmChannel
#[derive(Debug, PartialEq)]
pub enum GetAdmChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetAdmChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetAdmChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetAdmChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetAdmChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetAdmChannelError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetAdmChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetAdmChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetAdmChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetAdmChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetAdmChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetAdmChannelError {
    fn from(err: serde_json::error::Error) -> GetAdmChannelError {
        GetAdmChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAdmChannelError {
    fn from(err: CredentialsError) -> GetAdmChannelError {
        GetAdmChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAdmChannelError {
    fn from(err: HttpDispatchError) -> GetAdmChannelError {
        GetAdmChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAdmChannelError {
    fn from(err: io::Error) -> GetAdmChannelError {
        GetAdmChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAdmChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAdmChannelError {
    fn description(&self) -> &str {
        match *self {
            GetAdmChannelError::BadRequest(ref cause) => cause,
            GetAdmChannelError::Forbidden(ref cause) => cause,
            GetAdmChannelError::InternalServerError(ref cause) => cause,
            GetAdmChannelError::MethodNotAllowed(ref cause) => cause,
            GetAdmChannelError::NotFound(ref cause) => cause,
            GetAdmChannelError::TooManyRequests(ref cause) => cause,
            GetAdmChannelError::Validation(ref cause) => cause,
            GetAdmChannelError::Credentials(ref err) => err.description(),
            GetAdmChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetAdmChannelError::ParseError(ref cause) => cause,
            GetAdmChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetApnsChannel
#[derive(Debug, PartialEq)]
pub enum GetApnsChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetApnsChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetApnsChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetApnsChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetApnsChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetApnsChannelError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetApnsChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetApnsChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetApnsChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetApnsChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetApnsChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetApnsChannelError {
    fn from(err: serde_json::error::Error) -> GetApnsChannelError {
        GetApnsChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetApnsChannelError {
    fn from(err: CredentialsError) -> GetApnsChannelError {
        GetApnsChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetApnsChannelError {
    fn from(err: HttpDispatchError) -> GetApnsChannelError {
        GetApnsChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetApnsChannelError {
    fn from(err: io::Error) -> GetApnsChannelError {
        GetApnsChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetApnsChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApnsChannelError {
    fn description(&self) -> &str {
        match *self {
            GetApnsChannelError::BadRequest(ref cause) => cause,
            GetApnsChannelError::Forbidden(ref cause) => cause,
            GetApnsChannelError::InternalServerError(ref cause) => cause,
            GetApnsChannelError::MethodNotAllowed(ref cause) => cause,
            GetApnsChannelError::NotFound(ref cause) => cause,
            GetApnsChannelError::TooManyRequests(ref cause) => cause,
            GetApnsChannelError::Validation(ref cause) => cause,
            GetApnsChannelError::Credentials(ref err) => err.description(),
            GetApnsChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetApnsChannelError::ParseError(ref cause) => cause,
            GetApnsChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetApnsSandboxChannel
#[derive(Debug, PartialEq)]
pub enum GetApnsSandboxChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetApnsSandboxChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetApnsSandboxChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetApnsSandboxChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetApnsSandboxChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetApnsSandboxChannelError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "MethodNotAllowedException" => {
                    return GetApnsSandboxChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetApnsSandboxChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetApnsSandboxChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetApnsSandboxChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetApnsSandboxChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetApnsSandboxChannelError {
    fn from(err: serde_json::error::Error) -> GetApnsSandboxChannelError {
        GetApnsSandboxChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetApnsSandboxChannelError {
    fn from(err: CredentialsError) -> GetApnsSandboxChannelError {
        GetApnsSandboxChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetApnsSandboxChannelError {
    fn from(err: HttpDispatchError) -> GetApnsSandboxChannelError {
        GetApnsSandboxChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetApnsSandboxChannelError {
    fn from(err: io::Error) -> GetApnsSandboxChannelError {
        GetApnsSandboxChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetApnsSandboxChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApnsSandboxChannelError {
    fn description(&self) -> &str {
        match *self {
            GetApnsSandboxChannelError::BadRequest(ref cause) => cause,
            GetApnsSandboxChannelError::Forbidden(ref cause) => cause,
            GetApnsSandboxChannelError::InternalServerError(ref cause) => cause,
            GetApnsSandboxChannelError::MethodNotAllowed(ref cause) => cause,
            GetApnsSandboxChannelError::NotFound(ref cause) => cause,
            GetApnsSandboxChannelError::TooManyRequests(ref cause) => cause,
            GetApnsSandboxChannelError::Validation(ref cause) => cause,
            GetApnsSandboxChannelError::Credentials(ref err) => err.description(),
            GetApnsSandboxChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetApnsSandboxChannelError::ParseError(ref cause) => cause,
            GetApnsSandboxChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetApnsVoipChannel
#[derive(Debug, PartialEq)]
pub enum GetApnsVoipChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetApnsVoipChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetApnsVoipChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetApnsVoipChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetApnsVoipChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetApnsVoipChannelError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetApnsVoipChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetApnsVoipChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetApnsVoipChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetApnsVoipChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetApnsVoipChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetApnsVoipChannelError {
    fn from(err: serde_json::error::Error) -> GetApnsVoipChannelError {
        GetApnsVoipChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetApnsVoipChannelError {
    fn from(err: CredentialsError) -> GetApnsVoipChannelError {
        GetApnsVoipChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetApnsVoipChannelError {
    fn from(err: HttpDispatchError) -> GetApnsVoipChannelError {
        GetApnsVoipChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetApnsVoipChannelError {
    fn from(err: io::Error) -> GetApnsVoipChannelError {
        GetApnsVoipChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetApnsVoipChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApnsVoipChannelError {
    fn description(&self) -> &str {
        match *self {
            GetApnsVoipChannelError::BadRequest(ref cause) => cause,
            GetApnsVoipChannelError::Forbidden(ref cause) => cause,
            GetApnsVoipChannelError::InternalServerError(ref cause) => cause,
            GetApnsVoipChannelError::MethodNotAllowed(ref cause) => cause,
            GetApnsVoipChannelError::NotFound(ref cause) => cause,
            GetApnsVoipChannelError::TooManyRequests(ref cause) => cause,
            GetApnsVoipChannelError::Validation(ref cause) => cause,
            GetApnsVoipChannelError::Credentials(ref err) => err.description(),
            GetApnsVoipChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetApnsVoipChannelError::ParseError(ref cause) => cause,
            GetApnsVoipChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetApnsVoipSandboxChannel
#[derive(Debug, PartialEq)]
pub enum GetApnsVoipSandboxChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetApnsVoipSandboxChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetApnsVoipSandboxChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetApnsVoipSandboxChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetApnsVoipSandboxChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetApnsVoipSandboxChannelError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "MethodNotAllowedException" => {
                    return GetApnsVoipSandboxChannelError::MethodNotAllowed(String::from(
                        error_message,
                    ))
                }
                "NotFoundException" => {
                    return GetApnsVoipSandboxChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetApnsVoipSandboxChannelError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return GetApnsVoipSandboxChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetApnsVoipSandboxChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetApnsVoipSandboxChannelError {
    fn from(err: serde_json::error::Error) -> GetApnsVoipSandboxChannelError {
        GetApnsVoipSandboxChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetApnsVoipSandboxChannelError {
    fn from(err: CredentialsError) -> GetApnsVoipSandboxChannelError {
        GetApnsVoipSandboxChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetApnsVoipSandboxChannelError {
    fn from(err: HttpDispatchError) -> GetApnsVoipSandboxChannelError {
        GetApnsVoipSandboxChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetApnsVoipSandboxChannelError {
    fn from(err: io::Error) -> GetApnsVoipSandboxChannelError {
        GetApnsVoipSandboxChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetApnsVoipSandboxChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApnsVoipSandboxChannelError {
    fn description(&self) -> &str {
        match *self {
            GetApnsVoipSandboxChannelError::BadRequest(ref cause) => cause,
            GetApnsVoipSandboxChannelError::Forbidden(ref cause) => cause,
            GetApnsVoipSandboxChannelError::InternalServerError(ref cause) => cause,
            GetApnsVoipSandboxChannelError::MethodNotAllowed(ref cause) => cause,
            GetApnsVoipSandboxChannelError::NotFound(ref cause) => cause,
            GetApnsVoipSandboxChannelError::TooManyRequests(ref cause) => cause,
            GetApnsVoipSandboxChannelError::Validation(ref cause) => cause,
            GetApnsVoipSandboxChannelError::Credentials(ref err) => err.description(),
            GetApnsVoipSandboxChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetApnsVoipSandboxChannelError::ParseError(ref cause) => cause,
            GetApnsVoipSandboxChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetApp
#[derive(Debug, PartialEq)]
pub enum GetAppError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetAppError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetAppError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetAppError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => return GetAppError::Forbidden(String::from(error_message)),
                "InternalServerErrorException" => {
                    return GetAppError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetAppError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => return GetAppError::NotFound(String::from(error_message)),
                "TooManyRequestsException" => {
                    return GetAppError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => return GetAppError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return GetAppError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetAppError {
    fn from(err: serde_json::error::Error) -> GetAppError {
        GetAppError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAppError {
    fn from(err: CredentialsError) -> GetAppError {
        GetAppError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAppError {
    fn from(err: HttpDispatchError) -> GetAppError {
        GetAppError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAppError {
    fn from(err: io::Error) -> GetAppError {
        GetAppError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAppError {
    fn description(&self) -> &str {
        match *self {
            GetAppError::BadRequest(ref cause) => cause,
            GetAppError::Forbidden(ref cause) => cause,
            GetAppError::InternalServerError(ref cause) => cause,
            GetAppError::MethodNotAllowed(ref cause) => cause,
            GetAppError::NotFound(ref cause) => cause,
            GetAppError::TooManyRequests(ref cause) => cause,
            GetAppError::Validation(ref cause) => cause,
            GetAppError::Credentials(ref err) => err.description(),
            GetAppError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetAppError::ParseError(ref cause) => cause,
            GetAppError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetApplicationSettings
#[derive(Debug, PartialEq)]
pub enum GetApplicationSettingsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetApplicationSettingsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetApplicationSettingsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetApplicationSettingsError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetApplicationSettingsError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetApplicationSettingsError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "MethodNotAllowedException" => {
                    return GetApplicationSettingsError::MethodNotAllowed(String::from(
                        error_message,
                    ))
                }
                "NotFoundException" => {
                    return GetApplicationSettingsError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetApplicationSettingsError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetApplicationSettingsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetApplicationSettingsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetApplicationSettingsError {
    fn from(err: serde_json::error::Error) -> GetApplicationSettingsError {
        GetApplicationSettingsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetApplicationSettingsError {
    fn from(err: CredentialsError) -> GetApplicationSettingsError {
        GetApplicationSettingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetApplicationSettingsError {
    fn from(err: HttpDispatchError) -> GetApplicationSettingsError {
        GetApplicationSettingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetApplicationSettingsError {
    fn from(err: io::Error) -> GetApplicationSettingsError {
        GetApplicationSettingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetApplicationSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApplicationSettingsError {
    fn description(&self) -> &str {
        match *self {
            GetApplicationSettingsError::BadRequest(ref cause) => cause,
            GetApplicationSettingsError::Forbidden(ref cause) => cause,
            GetApplicationSettingsError::InternalServerError(ref cause) => cause,
            GetApplicationSettingsError::MethodNotAllowed(ref cause) => cause,
            GetApplicationSettingsError::NotFound(ref cause) => cause,
            GetApplicationSettingsError::TooManyRequests(ref cause) => cause,
            GetApplicationSettingsError::Validation(ref cause) => cause,
            GetApplicationSettingsError::Credentials(ref err) => err.description(),
            GetApplicationSettingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetApplicationSettingsError::ParseError(ref cause) => cause,
            GetApplicationSettingsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetApps
#[derive(Debug, PartialEq)]
pub enum GetAppsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetAppsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetAppsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetAppsError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => return GetAppsError::Forbidden(String::from(error_message)),
                "InternalServerErrorException" => {
                    return GetAppsError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetAppsError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => return GetAppsError::NotFound(String::from(error_message)),
                "TooManyRequestsException" => {
                    return GetAppsError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => return GetAppsError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return GetAppsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetAppsError {
    fn from(err: serde_json::error::Error) -> GetAppsError {
        GetAppsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAppsError {
    fn from(err: CredentialsError) -> GetAppsError {
        GetAppsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAppsError {
    fn from(err: HttpDispatchError) -> GetAppsError {
        GetAppsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAppsError {
    fn from(err: io::Error) -> GetAppsError {
        GetAppsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAppsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAppsError {
    fn description(&self) -> &str {
        match *self {
            GetAppsError::BadRequest(ref cause) => cause,
            GetAppsError::Forbidden(ref cause) => cause,
            GetAppsError::InternalServerError(ref cause) => cause,
            GetAppsError::MethodNotAllowed(ref cause) => cause,
            GetAppsError::NotFound(ref cause) => cause,
            GetAppsError::TooManyRequests(ref cause) => cause,
            GetAppsError::Validation(ref cause) => cause,
            GetAppsError::Credentials(ref err) => err.description(),
            GetAppsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetAppsError::ParseError(ref cause) => cause,
            GetAppsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetBaiduChannel
#[derive(Debug, PartialEq)]
pub enum GetBaiduChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetBaiduChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetBaiduChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetBaiduChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetBaiduChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetBaiduChannelError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetBaiduChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetBaiduChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetBaiduChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetBaiduChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetBaiduChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetBaiduChannelError {
    fn from(err: serde_json::error::Error) -> GetBaiduChannelError {
        GetBaiduChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetBaiduChannelError {
    fn from(err: CredentialsError) -> GetBaiduChannelError {
        GetBaiduChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBaiduChannelError {
    fn from(err: HttpDispatchError) -> GetBaiduChannelError {
        GetBaiduChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBaiduChannelError {
    fn from(err: io::Error) -> GetBaiduChannelError {
        GetBaiduChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBaiduChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBaiduChannelError {
    fn description(&self) -> &str {
        match *self {
            GetBaiduChannelError::BadRequest(ref cause) => cause,
            GetBaiduChannelError::Forbidden(ref cause) => cause,
            GetBaiduChannelError::InternalServerError(ref cause) => cause,
            GetBaiduChannelError::MethodNotAllowed(ref cause) => cause,
            GetBaiduChannelError::NotFound(ref cause) => cause,
            GetBaiduChannelError::TooManyRequests(ref cause) => cause,
            GetBaiduChannelError::Validation(ref cause) => cause,
            GetBaiduChannelError::Credentials(ref err) => err.description(),
            GetBaiduChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBaiduChannelError::ParseError(ref cause) => cause,
            GetBaiduChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetCampaign
#[derive(Debug, PartialEq)]
pub enum GetCampaignError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetCampaignError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetCampaignError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetCampaignError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetCampaignError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetCampaignError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetCampaignError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetCampaignError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetCampaignError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetCampaignError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetCampaignError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetCampaignError {
    fn from(err: serde_json::error::Error) -> GetCampaignError {
        GetCampaignError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCampaignError {
    fn from(err: CredentialsError) -> GetCampaignError {
        GetCampaignError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCampaignError {
    fn from(err: HttpDispatchError) -> GetCampaignError {
        GetCampaignError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCampaignError {
    fn from(err: io::Error) -> GetCampaignError {
        GetCampaignError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCampaignError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCampaignError {
    fn description(&self) -> &str {
        match *self {
            GetCampaignError::BadRequest(ref cause) => cause,
            GetCampaignError::Forbidden(ref cause) => cause,
            GetCampaignError::InternalServerError(ref cause) => cause,
            GetCampaignError::MethodNotAllowed(ref cause) => cause,
            GetCampaignError::NotFound(ref cause) => cause,
            GetCampaignError::TooManyRequests(ref cause) => cause,
            GetCampaignError::Validation(ref cause) => cause,
            GetCampaignError::Credentials(ref err) => err.description(),
            GetCampaignError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetCampaignError::ParseError(ref cause) => cause,
            GetCampaignError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetCampaignActivities
#[derive(Debug, PartialEq)]
pub enum GetCampaignActivitiesError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetCampaignActivitiesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetCampaignActivitiesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetCampaignActivitiesError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetCampaignActivitiesError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetCampaignActivitiesError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "MethodNotAllowedException" => {
                    return GetCampaignActivitiesError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetCampaignActivitiesError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetCampaignActivitiesError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetCampaignActivitiesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetCampaignActivitiesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetCampaignActivitiesError {
    fn from(err: serde_json::error::Error) -> GetCampaignActivitiesError {
        GetCampaignActivitiesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCampaignActivitiesError {
    fn from(err: CredentialsError) -> GetCampaignActivitiesError {
        GetCampaignActivitiesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCampaignActivitiesError {
    fn from(err: HttpDispatchError) -> GetCampaignActivitiesError {
        GetCampaignActivitiesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCampaignActivitiesError {
    fn from(err: io::Error) -> GetCampaignActivitiesError {
        GetCampaignActivitiesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCampaignActivitiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCampaignActivitiesError {
    fn description(&self) -> &str {
        match *self {
            GetCampaignActivitiesError::BadRequest(ref cause) => cause,
            GetCampaignActivitiesError::Forbidden(ref cause) => cause,
            GetCampaignActivitiesError::InternalServerError(ref cause) => cause,
            GetCampaignActivitiesError::MethodNotAllowed(ref cause) => cause,
            GetCampaignActivitiesError::NotFound(ref cause) => cause,
            GetCampaignActivitiesError::TooManyRequests(ref cause) => cause,
            GetCampaignActivitiesError::Validation(ref cause) => cause,
            GetCampaignActivitiesError::Credentials(ref err) => err.description(),
            GetCampaignActivitiesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetCampaignActivitiesError::ParseError(ref cause) => cause,
            GetCampaignActivitiesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetCampaignVersion
#[derive(Debug, PartialEq)]
pub enum GetCampaignVersionError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetCampaignVersionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetCampaignVersionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetCampaignVersionError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetCampaignVersionError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetCampaignVersionError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetCampaignVersionError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetCampaignVersionError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetCampaignVersionError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetCampaignVersionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetCampaignVersionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetCampaignVersionError {
    fn from(err: serde_json::error::Error) -> GetCampaignVersionError {
        GetCampaignVersionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCampaignVersionError {
    fn from(err: CredentialsError) -> GetCampaignVersionError {
        GetCampaignVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCampaignVersionError {
    fn from(err: HttpDispatchError) -> GetCampaignVersionError {
        GetCampaignVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCampaignVersionError {
    fn from(err: io::Error) -> GetCampaignVersionError {
        GetCampaignVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCampaignVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCampaignVersionError {
    fn description(&self) -> &str {
        match *self {
            GetCampaignVersionError::BadRequest(ref cause) => cause,
            GetCampaignVersionError::Forbidden(ref cause) => cause,
            GetCampaignVersionError::InternalServerError(ref cause) => cause,
            GetCampaignVersionError::MethodNotAllowed(ref cause) => cause,
            GetCampaignVersionError::NotFound(ref cause) => cause,
            GetCampaignVersionError::TooManyRequests(ref cause) => cause,
            GetCampaignVersionError::Validation(ref cause) => cause,
            GetCampaignVersionError::Credentials(ref err) => err.description(),
            GetCampaignVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetCampaignVersionError::ParseError(ref cause) => cause,
            GetCampaignVersionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetCampaignVersions
#[derive(Debug, PartialEq)]
pub enum GetCampaignVersionsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetCampaignVersionsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetCampaignVersionsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetCampaignVersionsError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetCampaignVersionsError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetCampaignVersionsError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "MethodNotAllowedException" => {
                    return GetCampaignVersionsError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetCampaignVersionsError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetCampaignVersionsError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetCampaignVersionsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetCampaignVersionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetCampaignVersionsError {
    fn from(err: serde_json::error::Error) -> GetCampaignVersionsError {
        GetCampaignVersionsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCampaignVersionsError {
    fn from(err: CredentialsError) -> GetCampaignVersionsError {
        GetCampaignVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCampaignVersionsError {
    fn from(err: HttpDispatchError) -> GetCampaignVersionsError {
        GetCampaignVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCampaignVersionsError {
    fn from(err: io::Error) -> GetCampaignVersionsError {
        GetCampaignVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCampaignVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCampaignVersionsError {
    fn description(&self) -> &str {
        match *self {
            GetCampaignVersionsError::BadRequest(ref cause) => cause,
            GetCampaignVersionsError::Forbidden(ref cause) => cause,
            GetCampaignVersionsError::InternalServerError(ref cause) => cause,
            GetCampaignVersionsError::MethodNotAllowed(ref cause) => cause,
            GetCampaignVersionsError::NotFound(ref cause) => cause,
            GetCampaignVersionsError::TooManyRequests(ref cause) => cause,
            GetCampaignVersionsError::Validation(ref cause) => cause,
            GetCampaignVersionsError::Credentials(ref err) => err.description(),
            GetCampaignVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetCampaignVersionsError::ParseError(ref cause) => cause,
            GetCampaignVersionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetCampaigns
#[derive(Debug, PartialEq)]
pub enum GetCampaignsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetCampaignsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetCampaignsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetCampaignsError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetCampaignsError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetCampaignsError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetCampaignsError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetCampaignsError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetCampaignsError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetCampaignsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetCampaignsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetCampaignsError {
    fn from(err: serde_json::error::Error) -> GetCampaignsError {
        GetCampaignsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCampaignsError {
    fn from(err: CredentialsError) -> GetCampaignsError {
        GetCampaignsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCampaignsError {
    fn from(err: HttpDispatchError) -> GetCampaignsError {
        GetCampaignsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCampaignsError {
    fn from(err: io::Error) -> GetCampaignsError {
        GetCampaignsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCampaignsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCampaignsError {
    fn description(&self) -> &str {
        match *self {
            GetCampaignsError::BadRequest(ref cause) => cause,
            GetCampaignsError::Forbidden(ref cause) => cause,
            GetCampaignsError::InternalServerError(ref cause) => cause,
            GetCampaignsError::MethodNotAllowed(ref cause) => cause,
            GetCampaignsError::NotFound(ref cause) => cause,
            GetCampaignsError::TooManyRequests(ref cause) => cause,
            GetCampaignsError::Validation(ref cause) => cause,
            GetCampaignsError::Credentials(ref err) => err.description(),
            GetCampaignsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetCampaignsError::ParseError(ref cause) => cause,
            GetCampaignsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetChannels
#[derive(Debug, PartialEq)]
pub enum GetChannelsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetChannelsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetChannelsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetChannelsError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetChannelsError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetChannelsError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetChannelsError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetChannelsError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetChannelsError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetChannelsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetChannelsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetChannelsError {
    fn from(err: serde_json::error::Error) -> GetChannelsError {
        GetChannelsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetChannelsError {
    fn from(err: CredentialsError) -> GetChannelsError {
        GetChannelsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetChannelsError {
    fn from(err: HttpDispatchError) -> GetChannelsError {
        GetChannelsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetChannelsError {
    fn from(err: io::Error) -> GetChannelsError {
        GetChannelsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetChannelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetChannelsError {
    fn description(&self) -> &str {
        match *self {
            GetChannelsError::BadRequest(ref cause) => cause,
            GetChannelsError::Forbidden(ref cause) => cause,
            GetChannelsError::InternalServerError(ref cause) => cause,
            GetChannelsError::MethodNotAllowed(ref cause) => cause,
            GetChannelsError::NotFound(ref cause) => cause,
            GetChannelsError::TooManyRequests(ref cause) => cause,
            GetChannelsError::Validation(ref cause) => cause,
            GetChannelsError::Credentials(ref err) => err.description(),
            GetChannelsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetChannelsError::ParseError(ref cause) => cause,
            GetChannelsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetEmailChannel
#[derive(Debug, PartialEq)]
pub enum GetEmailChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetEmailChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetEmailChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetEmailChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetEmailChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetEmailChannelError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetEmailChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetEmailChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetEmailChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetEmailChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetEmailChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetEmailChannelError {
    fn from(err: serde_json::error::Error) -> GetEmailChannelError {
        GetEmailChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetEmailChannelError {
    fn from(err: CredentialsError) -> GetEmailChannelError {
        GetEmailChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetEmailChannelError {
    fn from(err: HttpDispatchError) -> GetEmailChannelError {
        GetEmailChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetEmailChannelError {
    fn from(err: io::Error) -> GetEmailChannelError {
        GetEmailChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetEmailChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetEmailChannelError {
    fn description(&self) -> &str {
        match *self {
            GetEmailChannelError::BadRequest(ref cause) => cause,
            GetEmailChannelError::Forbidden(ref cause) => cause,
            GetEmailChannelError::InternalServerError(ref cause) => cause,
            GetEmailChannelError::MethodNotAllowed(ref cause) => cause,
            GetEmailChannelError::NotFound(ref cause) => cause,
            GetEmailChannelError::TooManyRequests(ref cause) => cause,
            GetEmailChannelError::Validation(ref cause) => cause,
            GetEmailChannelError::Credentials(ref err) => err.description(),
            GetEmailChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetEmailChannelError::ParseError(ref cause) => cause,
            GetEmailChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetEndpoint
#[derive(Debug, PartialEq)]
pub enum GetEndpointError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetEndpointError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetEndpointError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetEndpointError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetEndpointError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetEndpointError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetEndpointError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetEndpointError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetEndpointError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetEndpointError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetEndpointError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetEndpointError {
    fn from(err: serde_json::error::Error) -> GetEndpointError {
        GetEndpointError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetEndpointError {
    fn from(err: CredentialsError) -> GetEndpointError {
        GetEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetEndpointError {
    fn from(err: HttpDispatchError) -> GetEndpointError {
        GetEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetEndpointError {
    fn from(err: io::Error) -> GetEndpointError {
        GetEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetEndpointError {
    fn description(&self) -> &str {
        match *self {
            GetEndpointError::BadRequest(ref cause) => cause,
            GetEndpointError::Forbidden(ref cause) => cause,
            GetEndpointError::InternalServerError(ref cause) => cause,
            GetEndpointError::MethodNotAllowed(ref cause) => cause,
            GetEndpointError::NotFound(ref cause) => cause,
            GetEndpointError::TooManyRequests(ref cause) => cause,
            GetEndpointError::Validation(ref cause) => cause,
            GetEndpointError::Credentials(ref err) => err.description(),
            GetEndpointError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetEndpointError::ParseError(ref cause) => cause,
            GetEndpointError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetEventStream
#[derive(Debug, PartialEq)]
pub enum GetEventStreamError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetEventStreamError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetEventStreamError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetEventStreamError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetEventStreamError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetEventStreamError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetEventStreamError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetEventStreamError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetEventStreamError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetEventStreamError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetEventStreamError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetEventStreamError {
    fn from(err: serde_json::error::Error) -> GetEventStreamError {
        GetEventStreamError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetEventStreamError {
    fn from(err: CredentialsError) -> GetEventStreamError {
        GetEventStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetEventStreamError {
    fn from(err: HttpDispatchError) -> GetEventStreamError {
        GetEventStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetEventStreamError {
    fn from(err: io::Error) -> GetEventStreamError {
        GetEventStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetEventStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetEventStreamError {
    fn description(&self) -> &str {
        match *self {
            GetEventStreamError::BadRequest(ref cause) => cause,
            GetEventStreamError::Forbidden(ref cause) => cause,
            GetEventStreamError::InternalServerError(ref cause) => cause,
            GetEventStreamError::MethodNotAllowed(ref cause) => cause,
            GetEventStreamError::NotFound(ref cause) => cause,
            GetEventStreamError::TooManyRequests(ref cause) => cause,
            GetEventStreamError::Validation(ref cause) => cause,
            GetEventStreamError::Credentials(ref err) => err.description(),
            GetEventStreamError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetEventStreamError::ParseError(ref cause) => cause,
            GetEventStreamError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetExportJob
#[derive(Debug, PartialEq)]
pub enum GetExportJobError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetExportJobError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetExportJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetExportJobError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetExportJobError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetExportJobError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetExportJobError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetExportJobError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetExportJobError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetExportJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetExportJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetExportJobError {
    fn from(err: serde_json::error::Error) -> GetExportJobError {
        GetExportJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetExportJobError {
    fn from(err: CredentialsError) -> GetExportJobError {
        GetExportJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetExportJobError {
    fn from(err: HttpDispatchError) -> GetExportJobError {
        GetExportJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetExportJobError {
    fn from(err: io::Error) -> GetExportJobError {
        GetExportJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetExportJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetExportJobError {
    fn description(&self) -> &str {
        match *self {
            GetExportJobError::BadRequest(ref cause) => cause,
            GetExportJobError::Forbidden(ref cause) => cause,
            GetExportJobError::InternalServerError(ref cause) => cause,
            GetExportJobError::MethodNotAllowed(ref cause) => cause,
            GetExportJobError::NotFound(ref cause) => cause,
            GetExportJobError::TooManyRequests(ref cause) => cause,
            GetExportJobError::Validation(ref cause) => cause,
            GetExportJobError::Credentials(ref err) => err.description(),
            GetExportJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetExportJobError::ParseError(ref cause) => cause,
            GetExportJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetExportJobs
#[derive(Debug, PartialEq)]
pub enum GetExportJobsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetExportJobsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetExportJobsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetExportJobsError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetExportJobsError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetExportJobsError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetExportJobsError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetExportJobsError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetExportJobsError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetExportJobsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetExportJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetExportJobsError {
    fn from(err: serde_json::error::Error) -> GetExportJobsError {
        GetExportJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetExportJobsError {
    fn from(err: CredentialsError) -> GetExportJobsError {
        GetExportJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetExportJobsError {
    fn from(err: HttpDispatchError) -> GetExportJobsError {
        GetExportJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetExportJobsError {
    fn from(err: io::Error) -> GetExportJobsError {
        GetExportJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetExportJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetExportJobsError {
    fn description(&self) -> &str {
        match *self {
            GetExportJobsError::BadRequest(ref cause) => cause,
            GetExportJobsError::Forbidden(ref cause) => cause,
            GetExportJobsError::InternalServerError(ref cause) => cause,
            GetExportJobsError::MethodNotAllowed(ref cause) => cause,
            GetExportJobsError::NotFound(ref cause) => cause,
            GetExportJobsError::TooManyRequests(ref cause) => cause,
            GetExportJobsError::Validation(ref cause) => cause,
            GetExportJobsError::Credentials(ref err) => err.description(),
            GetExportJobsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetExportJobsError::ParseError(ref cause) => cause,
            GetExportJobsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetGcmChannel
#[derive(Debug, PartialEq)]
pub enum GetGcmChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetGcmChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetGcmChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetGcmChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetGcmChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetGcmChannelError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetGcmChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetGcmChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetGcmChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetGcmChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetGcmChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetGcmChannelError {
    fn from(err: serde_json::error::Error) -> GetGcmChannelError {
        GetGcmChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetGcmChannelError {
    fn from(err: CredentialsError) -> GetGcmChannelError {
        GetGcmChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetGcmChannelError {
    fn from(err: HttpDispatchError) -> GetGcmChannelError {
        GetGcmChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetGcmChannelError {
    fn from(err: io::Error) -> GetGcmChannelError {
        GetGcmChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetGcmChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGcmChannelError {
    fn description(&self) -> &str {
        match *self {
            GetGcmChannelError::BadRequest(ref cause) => cause,
            GetGcmChannelError::Forbidden(ref cause) => cause,
            GetGcmChannelError::InternalServerError(ref cause) => cause,
            GetGcmChannelError::MethodNotAllowed(ref cause) => cause,
            GetGcmChannelError::NotFound(ref cause) => cause,
            GetGcmChannelError::TooManyRequests(ref cause) => cause,
            GetGcmChannelError::Validation(ref cause) => cause,
            GetGcmChannelError::Credentials(ref err) => err.description(),
            GetGcmChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetGcmChannelError::ParseError(ref cause) => cause,
            GetGcmChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetImportJob
#[derive(Debug, PartialEq)]
pub enum GetImportJobError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetImportJobError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetImportJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetImportJobError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetImportJobError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetImportJobError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetImportJobError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetImportJobError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetImportJobError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetImportJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetImportJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetImportJobError {
    fn from(err: serde_json::error::Error) -> GetImportJobError {
        GetImportJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetImportJobError {
    fn from(err: CredentialsError) -> GetImportJobError {
        GetImportJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetImportJobError {
    fn from(err: HttpDispatchError) -> GetImportJobError {
        GetImportJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetImportJobError {
    fn from(err: io::Error) -> GetImportJobError {
        GetImportJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetImportJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetImportJobError {
    fn description(&self) -> &str {
        match *self {
            GetImportJobError::BadRequest(ref cause) => cause,
            GetImportJobError::Forbidden(ref cause) => cause,
            GetImportJobError::InternalServerError(ref cause) => cause,
            GetImportJobError::MethodNotAllowed(ref cause) => cause,
            GetImportJobError::NotFound(ref cause) => cause,
            GetImportJobError::TooManyRequests(ref cause) => cause,
            GetImportJobError::Validation(ref cause) => cause,
            GetImportJobError::Credentials(ref err) => err.description(),
            GetImportJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetImportJobError::ParseError(ref cause) => cause,
            GetImportJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetImportJobs
#[derive(Debug, PartialEq)]
pub enum GetImportJobsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetImportJobsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetImportJobsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetImportJobsError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetImportJobsError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetImportJobsError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetImportJobsError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetImportJobsError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetImportJobsError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetImportJobsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetImportJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetImportJobsError {
    fn from(err: serde_json::error::Error) -> GetImportJobsError {
        GetImportJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetImportJobsError {
    fn from(err: CredentialsError) -> GetImportJobsError {
        GetImportJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetImportJobsError {
    fn from(err: HttpDispatchError) -> GetImportJobsError {
        GetImportJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetImportJobsError {
    fn from(err: io::Error) -> GetImportJobsError {
        GetImportJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetImportJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetImportJobsError {
    fn description(&self) -> &str {
        match *self {
            GetImportJobsError::BadRequest(ref cause) => cause,
            GetImportJobsError::Forbidden(ref cause) => cause,
            GetImportJobsError::InternalServerError(ref cause) => cause,
            GetImportJobsError::MethodNotAllowed(ref cause) => cause,
            GetImportJobsError::NotFound(ref cause) => cause,
            GetImportJobsError::TooManyRequests(ref cause) => cause,
            GetImportJobsError::Validation(ref cause) => cause,
            GetImportJobsError::Credentials(ref err) => err.description(),
            GetImportJobsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetImportJobsError::ParseError(ref cause) => cause,
            GetImportJobsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetSegment
#[derive(Debug, PartialEq)]
pub enum GetSegmentError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetSegmentError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetSegmentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetSegmentError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetSegmentError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetSegmentError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetSegmentError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetSegmentError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetSegmentError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetSegmentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetSegmentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetSegmentError {
    fn from(err: serde_json::error::Error) -> GetSegmentError {
        GetSegmentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSegmentError {
    fn from(err: CredentialsError) -> GetSegmentError {
        GetSegmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSegmentError {
    fn from(err: HttpDispatchError) -> GetSegmentError {
        GetSegmentError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSegmentError {
    fn from(err: io::Error) -> GetSegmentError {
        GetSegmentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSegmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSegmentError {
    fn description(&self) -> &str {
        match *self {
            GetSegmentError::BadRequest(ref cause) => cause,
            GetSegmentError::Forbidden(ref cause) => cause,
            GetSegmentError::InternalServerError(ref cause) => cause,
            GetSegmentError::MethodNotAllowed(ref cause) => cause,
            GetSegmentError::NotFound(ref cause) => cause,
            GetSegmentError::TooManyRequests(ref cause) => cause,
            GetSegmentError::Validation(ref cause) => cause,
            GetSegmentError::Credentials(ref err) => err.description(),
            GetSegmentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetSegmentError::ParseError(ref cause) => cause,
            GetSegmentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetSegmentExportJobs
#[derive(Debug, PartialEq)]
pub enum GetSegmentExportJobsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetSegmentExportJobsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetSegmentExportJobsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetSegmentExportJobsError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetSegmentExportJobsError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetSegmentExportJobsError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "MethodNotAllowedException" => {
                    return GetSegmentExportJobsError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetSegmentExportJobsError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetSegmentExportJobsError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetSegmentExportJobsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetSegmentExportJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetSegmentExportJobsError {
    fn from(err: serde_json::error::Error) -> GetSegmentExportJobsError {
        GetSegmentExportJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSegmentExportJobsError {
    fn from(err: CredentialsError) -> GetSegmentExportJobsError {
        GetSegmentExportJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSegmentExportJobsError {
    fn from(err: HttpDispatchError) -> GetSegmentExportJobsError {
        GetSegmentExportJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSegmentExportJobsError {
    fn from(err: io::Error) -> GetSegmentExportJobsError {
        GetSegmentExportJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSegmentExportJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSegmentExportJobsError {
    fn description(&self) -> &str {
        match *self {
            GetSegmentExportJobsError::BadRequest(ref cause) => cause,
            GetSegmentExportJobsError::Forbidden(ref cause) => cause,
            GetSegmentExportJobsError::InternalServerError(ref cause) => cause,
            GetSegmentExportJobsError::MethodNotAllowed(ref cause) => cause,
            GetSegmentExportJobsError::NotFound(ref cause) => cause,
            GetSegmentExportJobsError::TooManyRequests(ref cause) => cause,
            GetSegmentExportJobsError::Validation(ref cause) => cause,
            GetSegmentExportJobsError::Credentials(ref err) => err.description(),
            GetSegmentExportJobsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetSegmentExportJobsError::ParseError(ref cause) => cause,
            GetSegmentExportJobsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetSegmentImportJobs
#[derive(Debug, PartialEq)]
pub enum GetSegmentImportJobsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetSegmentImportJobsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetSegmentImportJobsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetSegmentImportJobsError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetSegmentImportJobsError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetSegmentImportJobsError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "MethodNotAllowedException" => {
                    return GetSegmentImportJobsError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetSegmentImportJobsError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetSegmentImportJobsError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetSegmentImportJobsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetSegmentImportJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetSegmentImportJobsError {
    fn from(err: serde_json::error::Error) -> GetSegmentImportJobsError {
        GetSegmentImportJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSegmentImportJobsError {
    fn from(err: CredentialsError) -> GetSegmentImportJobsError {
        GetSegmentImportJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSegmentImportJobsError {
    fn from(err: HttpDispatchError) -> GetSegmentImportJobsError {
        GetSegmentImportJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSegmentImportJobsError {
    fn from(err: io::Error) -> GetSegmentImportJobsError {
        GetSegmentImportJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSegmentImportJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSegmentImportJobsError {
    fn description(&self) -> &str {
        match *self {
            GetSegmentImportJobsError::BadRequest(ref cause) => cause,
            GetSegmentImportJobsError::Forbidden(ref cause) => cause,
            GetSegmentImportJobsError::InternalServerError(ref cause) => cause,
            GetSegmentImportJobsError::MethodNotAllowed(ref cause) => cause,
            GetSegmentImportJobsError::NotFound(ref cause) => cause,
            GetSegmentImportJobsError::TooManyRequests(ref cause) => cause,
            GetSegmentImportJobsError::Validation(ref cause) => cause,
            GetSegmentImportJobsError::Credentials(ref err) => err.description(),
            GetSegmentImportJobsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetSegmentImportJobsError::ParseError(ref cause) => cause,
            GetSegmentImportJobsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetSegmentVersion
#[derive(Debug, PartialEq)]
pub enum GetSegmentVersionError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetSegmentVersionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetSegmentVersionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetSegmentVersionError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetSegmentVersionError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetSegmentVersionError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetSegmentVersionError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetSegmentVersionError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetSegmentVersionError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetSegmentVersionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetSegmentVersionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetSegmentVersionError {
    fn from(err: serde_json::error::Error) -> GetSegmentVersionError {
        GetSegmentVersionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSegmentVersionError {
    fn from(err: CredentialsError) -> GetSegmentVersionError {
        GetSegmentVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSegmentVersionError {
    fn from(err: HttpDispatchError) -> GetSegmentVersionError {
        GetSegmentVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSegmentVersionError {
    fn from(err: io::Error) -> GetSegmentVersionError {
        GetSegmentVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSegmentVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSegmentVersionError {
    fn description(&self) -> &str {
        match *self {
            GetSegmentVersionError::BadRequest(ref cause) => cause,
            GetSegmentVersionError::Forbidden(ref cause) => cause,
            GetSegmentVersionError::InternalServerError(ref cause) => cause,
            GetSegmentVersionError::MethodNotAllowed(ref cause) => cause,
            GetSegmentVersionError::NotFound(ref cause) => cause,
            GetSegmentVersionError::TooManyRequests(ref cause) => cause,
            GetSegmentVersionError::Validation(ref cause) => cause,
            GetSegmentVersionError::Credentials(ref err) => err.description(),
            GetSegmentVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetSegmentVersionError::ParseError(ref cause) => cause,
            GetSegmentVersionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetSegmentVersions
#[derive(Debug, PartialEq)]
pub enum GetSegmentVersionsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetSegmentVersionsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetSegmentVersionsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetSegmentVersionsError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetSegmentVersionsError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetSegmentVersionsError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetSegmentVersionsError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetSegmentVersionsError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetSegmentVersionsError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetSegmentVersionsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetSegmentVersionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetSegmentVersionsError {
    fn from(err: serde_json::error::Error) -> GetSegmentVersionsError {
        GetSegmentVersionsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSegmentVersionsError {
    fn from(err: CredentialsError) -> GetSegmentVersionsError {
        GetSegmentVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSegmentVersionsError {
    fn from(err: HttpDispatchError) -> GetSegmentVersionsError {
        GetSegmentVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSegmentVersionsError {
    fn from(err: io::Error) -> GetSegmentVersionsError {
        GetSegmentVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSegmentVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSegmentVersionsError {
    fn description(&self) -> &str {
        match *self {
            GetSegmentVersionsError::BadRequest(ref cause) => cause,
            GetSegmentVersionsError::Forbidden(ref cause) => cause,
            GetSegmentVersionsError::InternalServerError(ref cause) => cause,
            GetSegmentVersionsError::MethodNotAllowed(ref cause) => cause,
            GetSegmentVersionsError::NotFound(ref cause) => cause,
            GetSegmentVersionsError::TooManyRequests(ref cause) => cause,
            GetSegmentVersionsError::Validation(ref cause) => cause,
            GetSegmentVersionsError::Credentials(ref err) => err.description(),
            GetSegmentVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetSegmentVersionsError::ParseError(ref cause) => cause,
            GetSegmentVersionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetSegments
#[derive(Debug, PartialEq)]
pub enum GetSegmentsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetSegmentsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetSegmentsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetSegmentsError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetSegmentsError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetSegmentsError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetSegmentsError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetSegmentsError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetSegmentsError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetSegmentsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetSegmentsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetSegmentsError {
    fn from(err: serde_json::error::Error) -> GetSegmentsError {
        GetSegmentsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSegmentsError {
    fn from(err: CredentialsError) -> GetSegmentsError {
        GetSegmentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSegmentsError {
    fn from(err: HttpDispatchError) -> GetSegmentsError {
        GetSegmentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSegmentsError {
    fn from(err: io::Error) -> GetSegmentsError {
        GetSegmentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSegmentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSegmentsError {
    fn description(&self) -> &str {
        match *self {
            GetSegmentsError::BadRequest(ref cause) => cause,
            GetSegmentsError::Forbidden(ref cause) => cause,
            GetSegmentsError::InternalServerError(ref cause) => cause,
            GetSegmentsError::MethodNotAllowed(ref cause) => cause,
            GetSegmentsError::NotFound(ref cause) => cause,
            GetSegmentsError::TooManyRequests(ref cause) => cause,
            GetSegmentsError::Validation(ref cause) => cause,
            GetSegmentsError::Credentials(ref err) => err.description(),
            GetSegmentsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetSegmentsError::ParseError(ref cause) => cause,
            GetSegmentsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetSmsChannel
#[derive(Debug, PartialEq)]
pub enum GetSmsChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetSmsChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetSmsChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetSmsChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetSmsChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetSmsChannelError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetSmsChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetSmsChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetSmsChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetSmsChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetSmsChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetSmsChannelError {
    fn from(err: serde_json::error::Error) -> GetSmsChannelError {
        GetSmsChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSmsChannelError {
    fn from(err: CredentialsError) -> GetSmsChannelError {
        GetSmsChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSmsChannelError {
    fn from(err: HttpDispatchError) -> GetSmsChannelError {
        GetSmsChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSmsChannelError {
    fn from(err: io::Error) -> GetSmsChannelError {
        GetSmsChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSmsChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSmsChannelError {
    fn description(&self) -> &str {
        match *self {
            GetSmsChannelError::BadRequest(ref cause) => cause,
            GetSmsChannelError::Forbidden(ref cause) => cause,
            GetSmsChannelError::InternalServerError(ref cause) => cause,
            GetSmsChannelError::MethodNotAllowed(ref cause) => cause,
            GetSmsChannelError::NotFound(ref cause) => cause,
            GetSmsChannelError::TooManyRequests(ref cause) => cause,
            GetSmsChannelError::Validation(ref cause) => cause,
            GetSmsChannelError::Credentials(ref err) => err.description(),
            GetSmsChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetSmsChannelError::ParseError(ref cause) => cause,
            GetSmsChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetUserEndpoints
#[derive(Debug, PartialEq)]
pub enum GetUserEndpointsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetUserEndpointsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetUserEndpointsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetUserEndpointsError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return GetUserEndpointsError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return GetUserEndpointsError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return GetUserEndpointsError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetUserEndpointsError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetUserEndpointsError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetUserEndpointsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetUserEndpointsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetUserEndpointsError {
    fn from(err: serde_json::error::Error) -> GetUserEndpointsError {
        GetUserEndpointsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetUserEndpointsError {
    fn from(err: CredentialsError) -> GetUserEndpointsError {
        GetUserEndpointsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetUserEndpointsError {
    fn from(err: HttpDispatchError) -> GetUserEndpointsError {
        GetUserEndpointsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetUserEndpointsError {
    fn from(err: io::Error) -> GetUserEndpointsError {
        GetUserEndpointsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetUserEndpointsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUserEndpointsError {
    fn description(&self) -> &str {
        match *self {
            GetUserEndpointsError::BadRequest(ref cause) => cause,
            GetUserEndpointsError::Forbidden(ref cause) => cause,
            GetUserEndpointsError::InternalServerError(ref cause) => cause,
            GetUserEndpointsError::MethodNotAllowed(ref cause) => cause,
            GetUserEndpointsError::NotFound(ref cause) => cause,
            GetUserEndpointsError::TooManyRequests(ref cause) => cause,
            GetUserEndpointsError::Validation(ref cause) => cause,
            GetUserEndpointsError::Credentials(ref err) => err.description(),
            GetUserEndpointsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetUserEndpointsError::ParseError(ref cause) => cause,
            GetUserEndpointsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PhoneNumberValidate
#[derive(Debug, PartialEq)]
pub enum PhoneNumberValidateError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PhoneNumberValidateError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> PhoneNumberValidateError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return PhoneNumberValidateError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return PhoneNumberValidateError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return PhoneNumberValidateError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "MethodNotAllowedException" => {
                    return PhoneNumberValidateError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return PhoneNumberValidateError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return PhoneNumberValidateError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return PhoneNumberValidateError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return PhoneNumberValidateError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PhoneNumberValidateError {
    fn from(err: serde_json::error::Error) -> PhoneNumberValidateError {
        PhoneNumberValidateError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PhoneNumberValidateError {
    fn from(err: CredentialsError) -> PhoneNumberValidateError {
        PhoneNumberValidateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PhoneNumberValidateError {
    fn from(err: HttpDispatchError) -> PhoneNumberValidateError {
        PhoneNumberValidateError::HttpDispatch(err)
    }
}
impl From<io::Error> for PhoneNumberValidateError {
    fn from(err: io::Error) -> PhoneNumberValidateError {
        PhoneNumberValidateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PhoneNumberValidateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PhoneNumberValidateError {
    fn description(&self) -> &str {
        match *self {
            PhoneNumberValidateError::BadRequest(ref cause) => cause,
            PhoneNumberValidateError::Forbidden(ref cause) => cause,
            PhoneNumberValidateError::InternalServerError(ref cause) => cause,
            PhoneNumberValidateError::MethodNotAllowed(ref cause) => cause,
            PhoneNumberValidateError::NotFound(ref cause) => cause,
            PhoneNumberValidateError::TooManyRequests(ref cause) => cause,
            PhoneNumberValidateError::Validation(ref cause) => cause,
            PhoneNumberValidateError::Credentials(ref err) => err.description(),
            PhoneNumberValidateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PhoneNumberValidateError::ParseError(ref cause) => cause,
            PhoneNumberValidateError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutEventStream
#[derive(Debug, PartialEq)]
pub enum PutEventStreamError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PutEventStreamError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> PutEventStreamError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return PutEventStreamError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return PutEventStreamError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return PutEventStreamError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return PutEventStreamError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return PutEventStreamError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return PutEventStreamError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return PutEventStreamError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return PutEventStreamError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutEventStreamError {
    fn from(err: serde_json::error::Error) -> PutEventStreamError {
        PutEventStreamError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutEventStreamError {
    fn from(err: CredentialsError) -> PutEventStreamError {
        PutEventStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutEventStreamError {
    fn from(err: HttpDispatchError) -> PutEventStreamError {
        PutEventStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutEventStreamError {
    fn from(err: io::Error) -> PutEventStreamError {
        PutEventStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutEventStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutEventStreamError {
    fn description(&self) -> &str {
        match *self {
            PutEventStreamError::BadRequest(ref cause) => cause,
            PutEventStreamError::Forbidden(ref cause) => cause,
            PutEventStreamError::InternalServerError(ref cause) => cause,
            PutEventStreamError::MethodNotAllowed(ref cause) => cause,
            PutEventStreamError::NotFound(ref cause) => cause,
            PutEventStreamError::TooManyRequests(ref cause) => cause,
            PutEventStreamError::Validation(ref cause) => cause,
            PutEventStreamError::Credentials(ref err) => err.description(),
            PutEventStreamError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutEventStreamError::ParseError(ref cause) => cause,
            PutEventStreamError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RemoveAttributes
#[derive(Debug, PartialEq)]
pub enum RemoveAttributesError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl RemoveAttributesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RemoveAttributesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RemoveAttributesError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return RemoveAttributesError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return RemoveAttributesError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return RemoveAttributesError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return RemoveAttributesError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return RemoveAttributesError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return RemoveAttributesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return RemoveAttributesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RemoveAttributesError {
    fn from(err: serde_json::error::Error) -> RemoveAttributesError {
        RemoveAttributesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for RemoveAttributesError {
    fn from(err: CredentialsError) -> RemoveAttributesError {
        RemoveAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveAttributesError {
    fn from(err: HttpDispatchError) -> RemoveAttributesError {
        RemoveAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveAttributesError {
    fn from(err: io::Error) -> RemoveAttributesError {
        RemoveAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveAttributesError {
    fn description(&self) -> &str {
        match *self {
            RemoveAttributesError::BadRequest(ref cause) => cause,
            RemoveAttributesError::Forbidden(ref cause) => cause,
            RemoveAttributesError::InternalServerError(ref cause) => cause,
            RemoveAttributesError::MethodNotAllowed(ref cause) => cause,
            RemoveAttributesError::NotFound(ref cause) => cause,
            RemoveAttributesError::TooManyRequests(ref cause) => cause,
            RemoveAttributesError::Validation(ref cause) => cause,
            RemoveAttributesError::Credentials(ref err) => err.description(),
            RemoveAttributesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RemoveAttributesError::ParseError(ref cause) => cause,
            RemoveAttributesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by SendMessages
#[derive(Debug, PartialEq)]
pub enum SendMessagesError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl SendMessagesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> SendMessagesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return SendMessagesError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return SendMessagesError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return SendMessagesError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return SendMessagesError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return SendMessagesError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return SendMessagesError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return SendMessagesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return SendMessagesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for SendMessagesError {
    fn from(err: serde_json::error::Error) -> SendMessagesError {
        SendMessagesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for SendMessagesError {
    fn from(err: CredentialsError) -> SendMessagesError {
        SendMessagesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SendMessagesError {
    fn from(err: HttpDispatchError) -> SendMessagesError {
        SendMessagesError::HttpDispatch(err)
    }
}
impl From<io::Error> for SendMessagesError {
    fn from(err: io::Error) -> SendMessagesError {
        SendMessagesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SendMessagesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendMessagesError {
    fn description(&self) -> &str {
        match *self {
            SendMessagesError::BadRequest(ref cause) => cause,
            SendMessagesError::Forbidden(ref cause) => cause,
            SendMessagesError::InternalServerError(ref cause) => cause,
            SendMessagesError::MethodNotAllowed(ref cause) => cause,
            SendMessagesError::NotFound(ref cause) => cause,
            SendMessagesError::TooManyRequests(ref cause) => cause,
            SendMessagesError::Validation(ref cause) => cause,
            SendMessagesError::Credentials(ref err) => err.description(),
            SendMessagesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SendMessagesError::ParseError(ref cause) => cause,
            SendMessagesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by SendUsersMessages
#[derive(Debug, PartialEq)]
pub enum SendUsersMessagesError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl SendUsersMessagesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> SendUsersMessagesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return SendUsersMessagesError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return SendUsersMessagesError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return SendUsersMessagesError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return SendUsersMessagesError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return SendUsersMessagesError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return SendUsersMessagesError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return SendUsersMessagesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return SendUsersMessagesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for SendUsersMessagesError {
    fn from(err: serde_json::error::Error) -> SendUsersMessagesError {
        SendUsersMessagesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for SendUsersMessagesError {
    fn from(err: CredentialsError) -> SendUsersMessagesError {
        SendUsersMessagesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SendUsersMessagesError {
    fn from(err: HttpDispatchError) -> SendUsersMessagesError {
        SendUsersMessagesError::HttpDispatch(err)
    }
}
impl From<io::Error> for SendUsersMessagesError {
    fn from(err: io::Error) -> SendUsersMessagesError {
        SendUsersMessagesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SendUsersMessagesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendUsersMessagesError {
    fn description(&self) -> &str {
        match *self {
            SendUsersMessagesError::BadRequest(ref cause) => cause,
            SendUsersMessagesError::Forbidden(ref cause) => cause,
            SendUsersMessagesError::InternalServerError(ref cause) => cause,
            SendUsersMessagesError::MethodNotAllowed(ref cause) => cause,
            SendUsersMessagesError::NotFound(ref cause) => cause,
            SendUsersMessagesError::TooManyRequests(ref cause) => cause,
            SendUsersMessagesError::Validation(ref cause) => cause,
            SendUsersMessagesError::Credentials(ref err) => err.description(),
            SendUsersMessagesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SendUsersMessagesError::ParseError(ref cause) => cause,
            SendUsersMessagesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateAdmChannel
#[derive(Debug, PartialEq)]
pub enum UpdateAdmChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateAdmChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateAdmChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateAdmChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return UpdateAdmChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return UpdateAdmChannelError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return UpdateAdmChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return UpdateAdmChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return UpdateAdmChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateAdmChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateAdmChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateAdmChannelError {
    fn from(err: serde_json::error::Error) -> UpdateAdmChannelError {
        UpdateAdmChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateAdmChannelError {
    fn from(err: CredentialsError) -> UpdateAdmChannelError {
        UpdateAdmChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateAdmChannelError {
    fn from(err: HttpDispatchError) -> UpdateAdmChannelError {
        UpdateAdmChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateAdmChannelError {
    fn from(err: io::Error) -> UpdateAdmChannelError {
        UpdateAdmChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateAdmChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAdmChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateAdmChannelError::BadRequest(ref cause) => cause,
            UpdateAdmChannelError::Forbidden(ref cause) => cause,
            UpdateAdmChannelError::InternalServerError(ref cause) => cause,
            UpdateAdmChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateAdmChannelError::NotFound(ref cause) => cause,
            UpdateAdmChannelError::TooManyRequests(ref cause) => cause,
            UpdateAdmChannelError::Validation(ref cause) => cause,
            UpdateAdmChannelError::Credentials(ref err) => err.description(),
            UpdateAdmChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateAdmChannelError::ParseError(ref cause) => cause,
            UpdateAdmChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateApnsChannel
#[derive(Debug, PartialEq)]
pub enum UpdateApnsChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateApnsChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateApnsChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateApnsChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return UpdateApnsChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return UpdateApnsChannelError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return UpdateApnsChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return UpdateApnsChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return UpdateApnsChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateApnsChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateApnsChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateApnsChannelError {
    fn from(err: serde_json::error::Error) -> UpdateApnsChannelError {
        UpdateApnsChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateApnsChannelError {
    fn from(err: CredentialsError) -> UpdateApnsChannelError {
        UpdateApnsChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateApnsChannelError {
    fn from(err: HttpDispatchError) -> UpdateApnsChannelError {
        UpdateApnsChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateApnsChannelError {
    fn from(err: io::Error) -> UpdateApnsChannelError {
        UpdateApnsChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateApnsChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApnsChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateApnsChannelError::BadRequest(ref cause) => cause,
            UpdateApnsChannelError::Forbidden(ref cause) => cause,
            UpdateApnsChannelError::InternalServerError(ref cause) => cause,
            UpdateApnsChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateApnsChannelError::NotFound(ref cause) => cause,
            UpdateApnsChannelError::TooManyRequests(ref cause) => cause,
            UpdateApnsChannelError::Validation(ref cause) => cause,
            UpdateApnsChannelError::Credentials(ref err) => err.description(),
            UpdateApnsChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateApnsChannelError::ParseError(ref cause) => cause,
            UpdateApnsChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateApnsSandboxChannel
#[derive(Debug, PartialEq)]
pub enum UpdateApnsSandboxChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateApnsSandboxChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateApnsSandboxChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateApnsSandboxChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return UpdateApnsSandboxChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return UpdateApnsSandboxChannelError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "MethodNotAllowedException" => {
                    return UpdateApnsSandboxChannelError::MethodNotAllowed(String::from(
                        error_message,
                    ))
                }
                "NotFoundException" => {
                    return UpdateApnsSandboxChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return UpdateApnsSandboxChannelError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return UpdateApnsSandboxChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateApnsSandboxChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateApnsSandboxChannelError {
    fn from(err: serde_json::error::Error) -> UpdateApnsSandboxChannelError {
        UpdateApnsSandboxChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateApnsSandboxChannelError {
    fn from(err: CredentialsError) -> UpdateApnsSandboxChannelError {
        UpdateApnsSandboxChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateApnsSandboxChannelError {
    fn from(err: HttpDispatchError) -> UpdateApnsSandboxChannelError {
        UpdateApnsSandboxChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateApnsSandboxChannelError {
    fn from(err: io::Error) -> UpdateApnsSandboxChannelError {
        UpdateApnsSandboxChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateApnsSandboxChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApnsSandboxChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateApnsSandboxChannelError::BadRequest(ref cause) => cause,
            UpdateApnsSandboxChannelError::Forbidden(ref cause) => cause,
            UpdateApnsSandboxChannelError::InternalServerError(ref cause) => cause,
            UpdateApnsSandboxChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateApnsSandboxChannelError::NotFound(ref cause) => cause,
            UpdateApnsSandboxChannelError::TooManyRequests(ref cause) => cause,
            UpdateApnsSandboxChannelError::Validation(ref cause) => cause,
            UpdateApnsSandboxChannelError::Credentials(ref err) => err.description(),
            UpdateApnsSandboxChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateApnsSandboxChannelError::ParseError(ref cause) => cause,
            UpdateApnsSandboxChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateApnsVoipChannel
#[derive(Debug, PartialEq)]
pub enum UpdateApnsVoipChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateApnsVoipChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateApnsVoipChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateApnsVoipChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return UpdateApnsVoipChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return UpdateApnsVoipChannelError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "MethodNotAllowedException" => {
                    return UpdateApnsVoipChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return UpdateApnsVoipChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return UpdateApnsVoipChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateApnsVoipChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateApnsVoipChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateApnsVoipChannelError {
    fn from(err: serde_json::error::Error) -> UpdateApnsVoipChannelError {
        UpdateApnsVoipChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateApnsVoipChannelError {
    fn from(err: CredentialsError) -> UpdateApnsVoipChannelError {
        UpdateApnsVoipChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateApnsVoipChannelError {
    fn from(err: HttpDispatchError) -> UpdateApnsVoipChannelError {
        UpdateApnsVoipChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateApnsVoipChannelError {
    fn from(err: io::Error) -> UpdateApnsVoipChannelError {
        UpdateApnsVoipChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateApnsVoipChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApnsVoipChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateApnsVoipChannelError::BadRequest(ref cause) => cause,
            UpdateApnsVoipChannelError::Forbidden(ref cause) => cause,
            UpdateApnsVoipChannelError::InternalServerError(ref cause) => cause,
            UpdateApnsVoipChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateApnsVoipChannelError::NotFound(ref cause) => cause,
            UpdateApnsVoipChannelError::TooManyRequests(ref cause) => cause,
            UpdateApnsVoipChannelError::Validation(ref cause) => cause,
            UpdateApnsVoipChannelError::Credentials(ref err) => err.description(),
            UpdateApnsVoipChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateApnsVoipChannelError::ParseError(ref cause) => cause,
            UpdateApnsVoipChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateApnsVoipSandboxChannel
#[derive(Debug, PartialEq)]
pub enum UpdateApnsVoipSandboxChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateApnsVoipSandboxChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateApnsVoipSandboxChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateApnsVoipSandboxChannelError::BadRequest(String::from(
                        error_message,
                    ))
                }
                "ForbiddenException" => {
                    return UpdateApnsVoipSandboxChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return UpdateApnsVoipSandboxChannelError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "MethodNotAllowedException" => {
                    return UpdateApnsVoipSandboxChannelError::MethodNotAllowed(String::from(
                        error_message,
                    ))
                }
                "NotFoundException" => {
                    return UpdateApnsVoipSandboxChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return UpdateApnsVoipSandboxChannelError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return UpdateApnsVoipSandboxChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateApnsVoipSandboxChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateApnsVoipSandboxChannelError {
    fn from(err: serde_json::error::Error) -> UpdateApnsVoipSandboxChannelError {
        UpdateApnsVoipSandboxChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateApnsVoipSandboxChannelError {
    fn from(err: CredentialsError) -> UpdateApnsVoipSandboxChannelError {
        UpdateApnsVoipSandboxChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateApnsVoipSandboxChannelError {
    fn from(err: HttpDispatchError) -> UpdateApnsVoipSandboxChannelError {
        UpdateApnsVoipSandboxChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateApnsVoipSandboxChannelError {
    fn from(err: io::Error) -> UpdateApnsVoipSandboxChannelError {
        UpdateApnsVoipSandboxChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateApnsVoipSandboxChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApnsVoipSandboxChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateApnsVoipSandboxChannelError::BadRequest(ref cause) => cause,
            UpdateApnsVoipSandboxChannelError::Forbidden(ref cause) => cause,
            UpdateApnsVoipSandboxChannelError::InternalServerError(ref cause) => cause,
            UpdateApnsVoipSandboxChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateApnsVoipSandboxChannelError::NotFound(ref cause) => cause,
            UpdateApnsVoipSandboxChannelError::TooManyRequests(ref cause) => cause,
            UpdateApnsVoipSandboxChannelError::Validation(ref cause) => cause,
            UpdateApnsVoipSandboxChannelError::Credentials(ref err) => err.description(),
            UpdateApnsVoipSandboxChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateApnsVoipSandboxChannelError::ParseError(ref cause) => cause,
            UpdateApnsVoipSandboxChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateApplicationSettings
#[derive(Debug, PartialEq)]
pub enum UpdateApplicationSettingsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateApplicationSettingsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateApplicationSettingsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateApplicationSettingsError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return UpdateApplicationSettingsError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return UpdateApplicationSettingsError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "MethodNotAllowedException" => {
                    return UpdateApplicationSettingsError::MethodNotAllowed(String::from(
                        error_message,
                    ))
                }
                "NotFoundException" => {
                    return UpdateApplicationSettingsError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return UpdateApplicationSettingsError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return UpdateApplicationSettingsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateApplicationSettingsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateApplicationSettingsError {
    fn from(err: serde_json::error::Error) -> UpdateApplicationSettingsError {
        UpdateApplicationSettingsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateApplicationSettingsError {
    fn from(err: CredentialsError) -> UpdateApplicationSettingsError {
        UpdateApplicationSettingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateApplicationSettingsError {
    fn from(err: HttpDispatchError) -> UpdateApplicationSettingsError {
        UpdateApplicationSettingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateApplicationSettingsError {
    fn from(err: io::Error) -> UpdateApplicationSettingsError {
        UpdateApplicationSettingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateApplicationSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApplicationSettingsError {
    fn description(&self) -> &str {
        match *self {
            UpdateApplicationSettingsError::BadRequest(ref cause) => cause,
            UpdateApplicationSettingsError::Forbidden(ref cause) => cause,
            UpdateApplicationSettingsError::InternalServerError(ref cause) => cause,
            UpdateApplicationSettingsError::MethodNotAllowed(ref cause) => cause,
            UpdateApplicationSettingsError::NotFound(ref cause) => cause,
            UpdateApplicationSettingsError::TooManyRequests(ref cause) => cause,
            UpdateApplicationSettingsError::Validation(ref cause) => cause,
            UpdateApplicationSettingsError::Credentials(ref err) => err.description(),
            UpdateApplicationSettingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateApplicationSettingsError::ParseError(ref cause) => cause,
            UpdateApplicationSettingsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateBaiduChannel
#[derive(Debug, PartialEq)]
pub enum UpdateBaiduChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateBaiduChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateBaiduChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateBaiduChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return UpdateBaiduChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return UpdateBaiduChannelError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return UpdateBaiduChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return UpdateBaiduChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return UpdateBaiduChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateBaiduChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateBaiduChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateBaiduChannelError {
    fn from(err: serde_json::error::Error) -> UpdateBaiduChannelError {
        UpdateBaiduChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateBaiduChannelError {
    fn from(err: CredentialsError) -> UpdateBaiduChannelError {
        UpdateBaiduChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateBaiduChannelError {
    fn from(err: HttpDispatchError) -> UpdateBaiduChannelError {
        UpdateBaiduChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateBaiduChannelError {
    fn from(err: io::Error) -> UpdateBaiduChannelError {
        UpdateBaiduChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateBaiduChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateBaiduChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateBaiduChannelError::BadRequest(ref cause) => cause,
            UpdateBaiduChannelError::Forbidden(ref cause) => cause,
            UpdateBaiduChannelError::InternalServerError(ref cause) => cause,
            UpdateBaiduChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateBaiduChannelError::NotFound(ref cause) => cause,
            UpdateBaiduChannelError::TooManyRequests(ref cause) => cause,
            UpdateBaiduChannelError::Validation(ref cause) => cause,
            UpdateBaiduChannelError::Credentials(ref err) => err.description(),
            UpdateBaiduChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateBaiduChannelError::ParseError(ref cause) => cause,
            UpdateBaiduChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateCampaign
#[derive(Debug, PartialEq)]
pub enum UpdateCampaignError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateCampaignError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateCampaignError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateCampaignError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return UpdateCampaignError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return UpdateCampaignError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return UpdateCampaignError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return UpdateCampaignError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return UpdateCampaignError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateCampaignError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateCampaignError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateCampaignError {
    fn from(err: serde_json::error::Error) -> UpdateCampaignError {
        UpdateCampaignError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateCampaignError {
    fn from(err: CredentialsError) -> UpdateCampaignError {
        UpdateCampaignError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateCampaignError {
    fn from(err: HttpDispatchError) -> UpdateCampaignError {
        UpdateCampaignError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateCampaignError {
    fn from(err: io::Error) -> UpdateCampaignError {
        UpdateCampaignError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateCampaignError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCampaignError {
    fn description(&self) -> &str {
        match *self {
            UpdateCampaignError::BadRequest(ref cause) => cause,
            UpdateCampaignError::Forbidden(ref cause) => cause,
            UpdateCampaignError::InternalServerError(ref cause) => cause,
            UpdateCampaignError::MethodNotAllowed(ref cause) => cause,
            UpdateCampaignError::NotFound(ref cause) => cause,
            UpdateCampaignError::TooManyRequests(ref cause) => cause,
            UpdateCampaignError::Validation(ref cause) => cause,
            UpdateCampaignError::Credentials(ref err) => err.description(),
            UpdateCampaignError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateCampaignError::ParseError(ref cause) => cause,
            UpdateCampaignError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateEmailChannel
#[derive(Debug, PartialEq)]
pub enum UpdateEmailChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateEmailChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateEmailChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateEmailChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return UpdateEmailChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return UpdateEmailChannelError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return UpdateEmailChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return UpdateEmailChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return UpdateEmailChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateEmailChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateEmailChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateEmailChannelError {
    fn from(err: serde_json::error::Error) -> UpdateEmailChannelError {
        UpdateEmailChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateEmailChannelError {
    fn from(err: CredentialsError) -> UpdateEmailChannelError {
        UpdateEmailChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateEmailChannelError {
    fn from(err: HttpDispatchError) -> UpdateEmailChannelError {
        UpdateEmailChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateEmailChannelError {
    fn from(err: io::Error) -> UpdateEmailChannelError {
        UpdateEmailChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateEmailChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateEmailChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateEmailChannelError::BadRequest(ref cause) => cause,
            UpdateEmailChannelError::Forbidden(ref cause) => cause,
            UpdateEmailChannelError::InternalServerError(ref cause) => cause,
            UpdateEmailChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateEmailChannelError::NotFound(ref cause) => cause,
            UpdateEmailChannelError::TooManyRequests(ref cause) => cause,
            UpdateEmailChannelError::Validation(ref cause) => cause,
            UpdateEmailChannelError::Credentials(ref err) => err.description(),
            UpdateEmailChannelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateEmailChannelError::ParseError(ref cause) => cause,
            UpdateEmailChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateEndpoint
#[derive(Debug, PartialEq)]
pub enum UpdateEndpointError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateEndpointError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateEndpointError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateEndpointError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return UpdateEndpointError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return UpdateEndpointError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return UpdateEndpointError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return UpdateEndpointError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return UpdateEndpointError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateEndpointError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateEndpointError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateEndpointError {
    fn from(err: serde_json::error::Error) -> UpdateEndpointError {
        UpdateEndpointError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateEndpointError {
    fn from(err: CredentialsError) -> UpdateEndpointError {
        UpdateEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateEndpointError {
    fn from(err: HttpDispatchError) -> UpdateEndpointError {
        UpdateEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateEndpointError {
    fn from(err: io::Error) -> UpdateEndpointError {
        UpdateEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateEndpointError {
    fn description(&self) -> &str {
        match *self {
            UpdateEndpointError::BadRequest(ref cause) => cause,
            UpdateEndpointError::Forbidden(ref cause) => cause,
            UpdateEndpointError::InternalServerError(ref cause) => cause,
            UpdateEndpointError::MethodNotAllowed(ref cause) => cause,
            UpdateEndpointError::NotFound(ref cause) => cause,
            UpdateEndpointError::TooManyRequests(ref cause) => cause,
            UpdateEndpointError::Validation(ref cause) => cause,
            UpdateEndpointError::Credentials(ref err) => err.description(),
            UpdateEndpointError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateEndpointError::ParseError(ref cause) => cause,
            UpdateEndpointError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateEndpointsBatch
#[derive(Debug, PartialEq)]
pub enum UpdateEndpointsBatchError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateEndpointsBatchError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateEndpointsBatchError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateEndpointsBatchError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return UpdateEndpointsBatchError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return UpdateEndpointsBatchError::InternalServerError(String::from(
                        error_message,
                    ))
                }
                "MethodNotAllowedException" => {
                    return UpdateEndpointsBatchError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return UpdateEndpointsBatchError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return UpdateEndpointsBatchError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateEndpointsBatchError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateEndpointsBatchError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateEndpointsBatchError {
    fn from(err: serde_json::error::Error) -> UpdateEndpointsBatchError {
        UpdateEndpointsBatchError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateEndpointsBatchError {
    fn from(err: CredentialsError) -> UpdateEndpointsBatchError {
        UpdateEndpointsBatchError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateEndpointsBatchError {
    fn from(err: HttpDispatchError) -> UpdateEndpointsBatchError {
        UpdateEndpointsBatchError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateEndpointsBatchError {
    fn from(err: io::Error) -> UpdateEndpointsBatchError {
        UpdateEndpointsBatchError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateEndpointsBatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateEndpointsBatchError {
    fn description(&self) -> &str {
        match *self {
            UpdateEndpointsBatchError::BadRequest(ref cause) => cause,
            UpdateEndpointsBatchError::Forbidden(ref cause) => cause,
            UpdateEndpointsBatchError::InternalServerError(ref cause) => cause,
            UpdateEndpointsBatchError::MethodNotAllowed(ref cause) => cause,
            UpdateEndpointsBatchError::NotFound(ref cause) => cause,
            UpdateEndpointsBatchError::TooManyRequests(ref cause) => cause,
            UpdateEndpointsBatchError::Validation(ref cause) => cause,
            UpdateEndpointsBatchError::Credentials(ref err) => err.description(),
            UpdateEndpointsBatchError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateEndpointsBatchError::ParseError(ref cause) => cause,
            UpdateEndpointsBatchError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateGcmChannel
#[derive(Debug, PartialEq)]
pub enum UpdateGcmChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateGcmChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateGcmChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateGcmChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return UpdateGcmChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return UpdateGcmChannelError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return UpdateGcmChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return UpdateGcmChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return UpdateGcmChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateGcmChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateGcmChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateGcmChannelError {
    fn from(err: serde_json::error::Error) -> UpdateGcmChannelError {
        UpdateGcmChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateGcmChannelError {
    fn from(err: CredentialsError) -> UpdateGcmChannelError {
        UpdateGcmChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateGcmChannelError {
    fn from(err: HttpDispatchError) -> UpdateGcmChannelError {
        UpdateGcmChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateGcmChannelError {
    fn from(err: io::Error) -> UpdateGcmChannelError {
        UpdateGcmChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateGcmChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGcmChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateGcmChannelError::BadRequest(ref cause) => cause,
            UpdateGcmChannelError::Forbidden(ref cause) => cause,
            UpdateGcmChannelError::InternalServerError(ref cause) => cause,
            UpdateGcmChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateGcmChannelError::NotFound(ref cause) => cause,
            UpdateGcmChannelError::TooManyRequests(ref cause) => cause,
            UpdateGcmChannelError::Validation(ref cause) => cause,
            UpdateGcmChannelError::Credentials(ref err) => err.description(),
            UpdateGcmChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateGcmChannelError::ParseError(ref cause) => cause,
            UpdateGcmChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateSegment
#[derive(Debug, PartialEq)]
pub enum UpdateSegmentError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateSegmentError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateSegmentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateSegmentError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return UpdateSegmentError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return UpdateSegmentError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return UpdateSegmentError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return UpdateSegmentError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return UpdateSegmentError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateSegmentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateSegmentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateSegmentError {
    fn from(err: serde_json::error::Error) -> UpdateSegmentError {
        UpdateSegmentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateSegmentError {
    fn from(err: CredentialsError) -> UpdateSegmentError {
        UpdateSegmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateSegmentError {
    fn from(err: HttpDispatchError) -> UpdateSegmentError {
        UpdateSegmentError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateSegmentError {
    fn from(err: io::Error) -> UpdateSegmentError {
        UpdateSegmentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateSegmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateSegmentError {
    fn description(&self) -> &str {
        match *self {
            UpdateSegmentError::BadRequest(ref cause) => cause,
            UpdateSegmentError::Forbidden(ref cause) => cause,
            UpdateSegmentError::InternalServerError(ref cause) => cause,
            UpdateSegmentError::MethodNotAllowed(ref cause) => cause,
            UpdateSegmentError::NotFound(ref cause) => cause,
            UpdateSegmentError::TooManyRequests(ref cause) => cause,
            UpdateSegmentError::Validation(ref cause) => cause,
            UpdateSegmentError::Credentials(ref err) => err.description(),
            UpdateSegmentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateSegmentError::ParseError(ref cause) => cause,
            UpdateSegmentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateSmsChannel
#[derive(Debug, PartialEq)]
pub enum UpdateSmsChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateSmsChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateSmsChannelError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateSmsChannelError::BadRequest(String::from(error_message))
                }
                "ForbiddenException" => {
                    return UpdateSmsChannelError::Forbidden(String::from(error_message))
                }
                "InternalServerErrorException" => {
                    return UpdateSmsChannelError::InternalServerError(String::from(error_message))
                }
                "MethodNotAllowedException" => {
                    return UpdateSmsChannelError::MethodNotAllowed(String::from(error_message))
                }
                "NotFoundException" => {
                    return UpdateSmsChannelError::NotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return UpdateSmsChannelError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateSmsChannelError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateSmsChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateSmsChannelError {
    fn from(err: serde_json::error::Error) -> UpdateSmsChannelError {
        UpdateSmsChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateSmsChannelError {
    fn from(err: CredentialsError) -> UpdateSmsChannelError {
        UpdateSmsChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateSmsChannelError {
    fn from(err: HttpDispatchError) -> UpdateSmsChannelError {
        UpdateSmsChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateSmsChannelError {
    fn from(err: io::Error) -> UpdateSmsChannelError {
        UpdateSmsChannelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateSmsChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateSmsChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateSmsChannelError::BadRequest(ref cause) => cause,
            UpdateSmsChannelError::Forbidden(ref cause) => cause,
            UpdateSmsChannelError::InternalServerError(ref cause) => cause,
            UpdateSmsChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateSmsChannelError::NotFound(ref cause) => cause,
            UpdateSmsChannelError::TooManyRequests(ref cause) => cause,
            UpdateSmsChannelError::Validation(ref cause) => cause,
            UpdateSmsChannelError::Credentials(ref err) => err.description(),
            UpdateSmsChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateSmsChannelError::ParseError(ref cause) => cause,
            UpdateSmsChannelError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the Amazon Pinpoint API. Amazon Pinpoint clients implement this trait.
pub trait Pinpoint {
    /// <p>Creates or updates an app.</p>
    fn create_app(
        &self,
        input: CreateAppRequest,
    ) -> RusotoFuture<CreateAppResponse, CreateAppError>;

    /// <p>Creates or updates a campaign.</p>
    fn create_campaign(
        &self,
        input: CreateCampaignRequest,
    ) -> RusotoFuture<CreateCampaignResponse, CreateCampaignError>;

    /// <p>Creates an export job.</p>
    fn create_export_job(
        &self,
        input: CreateExportJobRequest,
    ) -> RusotoFuture<CreateExportJobResponse, CreateExportJobError>;

    /// <p>Creates or updates an import job.</p>
    fn create_import_job(
        &self,
        input: CreateImportJobRequest,
    ) -> RusotoFuture<CreateImportJobResponse, CreateImportJobError>;

    /// <p>Used to create or update a segment.</p>
    fn create_segment(
        &self,
        input: CreateSegmentRequest,
    ) -> RusotoFuture<CreateSegmentResponse, CreateSegmentError>;

    /// <p>Delete an ADM channel.</p>
    fn delete_adm_channel(
        &self,
        input: DeleteAdmChannelRequest,
    ) -> RusotoFuture<DeleteAdmChannelResponse, DeleteAdmChannelError>;

    /// <p>Deletes the APNs channel for an app.</p>
    fn delete_apns_channel(
        &self,
        input: DeleteApnsChannelRequest,
    ) -> RusotoFuture<DeleteApnsChannelResponse, DeleteApnsChannelError>;

    /// <p>Delete an APNS sandbox channel.</p>
    fn delete_apns_sandbox_channel(
        &self,
        input: DeleteApnsSandboxChannelRequest,
    ) -> RusotoFuture<DeleteApnsSandboxChannelResponse, DeleteApnsSandboxChannelError>;

    /// <p>Delete an APNS VoIP channel</p>
    fn delete_apns_voip_channel(
        &self,
        input: DeleteApnsVoipChannelRequest,
    ) -> RusotoFuture<DeleteApnsVoipChannelResponse, DeleteApnsVoipChannelError>;

    /// <p>Delete an APNS VoIP sandbox channel</p>
    fn delete_apns_voip_sandbox_channel(
        &self,
        input: DeleteApnsVoipSandboxChannelRequest,
    ) -> RusotoFuture<DeleteApnsVoipSandboxChannelResponse, DeleteApnsVoipSandboxChannelError>;

    /// <p>Deletes an app.</p>
    fn delete_app(
        &self,
        input: DeleteAppRequest,
    ) -> RusotoFuture<DeleteAppResponse, DeleteAppError>;

    /// <p>Delete a BAIDU GCM channel</p>
    fn delete_baidu_channel(
        &self,
        input: DeleteBaiduChannelRequest,
    ) -> RusotoFuture<DeleteBaiduChannelResponse, DeleteBaiduChannelError>;

    /// <p>Deletes a campaign.</p>
    fn delete_campaign(
        &self,
        input: DeleteCampaignRequest,
    ) -> RusotoFuture<DeleteCampaignResponse, DeleteCampaignError>;

    /// <p>Delete an email channel.</p>
    fn delete_email_channel(
        &self,
        input: DeleteEmailChannelRequest,
    ) -> RusotoFuture<DeleteEmailChannelResponse, DeleteEmailChannelError>;

    /// <p>Deletes an endpoint.</p>
    fn delete_endpoint(
        &self,
        input: DeleteEndpointRequest,
    ) -> RusotoFuture<DeleteEndpointResponse, DeleteEndpointError>;

    /// <p>Deletes the event stream for an app.</p>
    fn delete_event_stream(
        &self,
        input: DeleteEventStreamRequest,
    ) -> RusotoFuture<DeleteEventStreamResponse, DeleteEventStreamError>;

    /// <p>Deletes the GCM channel for an app.</p>
    fn delete_gcm_channel(
        &self,
        input: DeleteGcmChannelRequest,
    ) -> RusotoFuture<DeleteGcmChannelResponse, DeleteGcmChannelError>;

    /// <p>Deletes a segment.</p>
    fn delete_segment(
        &self,
        input: DeleteSegmentRequest,
    ) -> RusotoFuture<DeleteSegmentResponse, DeleteSegmentError>;

    /// <p>Delete an SMS channel.</p>
    fn delete_sms_channel(
        &self,
        input: DeleteSmsChannelRequest,
    ) -> RusotoFuture<DeleteSmsChannelResponse, DeleteSmsChannelError>;

    /// <p>Deletes endpoints associated with an user id.</p>
    fn delete_user_endpoints(
        &self,
        input: DeleteUserEndpointsRequest,
    ) -> RusotoFuture<DeleteUserEndpointsResponse, DeleteUserEndpointsError>;

    /// <p>Get an ADM channel.</p>
    fn get_adm_channel(
        &self,
        input: GetAdmChannelRequest,
    ) -> RusotoFuture<GetAdmChannelResponse, GetAdmChannelError>;

    /// <p>Returns information about the APNs channel for an app.</p>
    fn get_apns_channel(
        &self,
        input: GetApnsChannelRequest,
    ) -> RusotoFuture<GetApnsChannelResponse, GetApnsChannelError>;

    /// <p>Get an APNS sandbox channel.</p>
    fn get_apns_sandbox_channel(
        &self,
        input: GetApnsSandboxChannelRequest,
    ) -> RusotoFuture<GetApnsSandboxChannelResponse, GetApnsSandboxChannelError>;

    /// <p>Get an APNS VoIP channel</p>
    fn get_apns_voip_channel(
        &self,
        input: GetApnsVoipChannelRequest,
    ) -> RusotoFuture<GetApnsVoipChannelResponse, GetApnsVoipChannelError>;

    /// <p>Get an APNS VoIPSandbox channel</p>
    fn get_apns_voip_sandbox_channel(
        &self,
        input: GetApnsVoipSandboxChannelRequest,
    ) -> RusotoFuture<GetApnsVoipSandboxChannelResponse, GetApnsVoipSandboxChannelError>;

    /// <p>Returns information about an app.</p>
    fn get_app(&self, input: GetAppRequest) -> RusotoFuture<GetAppResponse, GetAppError>;

    /// <p>Used to request the settings for an app.</p>
    fn get_application_settings(
        &self,
        input: GetApplicationSettingsRequest,
    ) -> RusotoFuture<GetApplicationSettingsResponse, GetApplicationSettingsError>;

    /// <p>Returns information about your apps.</p>
    fn get_apps(&self, input: GetAppsRequest) -> RusotoFuture<GetAppsResponse, GetAppsError>;

    /// <p>Get a BAIDU GCM channel</p>
    fn get_baidu_channel(
        &self,
        input: GetBaiduChannelRequest,
    ) -> RusotoFuture<GetBaiduChannelResponse, GetBaiduChannelError>;

    /// <p>Returns information about a campaign.</p>
    fn get_campaign(
        &self,
        input: GetCampaignRequest,
    ) -> RusotoFuture<GetCampaignResponse, GetCampaignError>;

    /// <p>Returns information about the activity performed by a campaign.</p>
    fn get_campaign_activities(
        &self,
        input: GetCampaignActivitiesRequest,
    ) -> RusotoFuture<GetCampaignActivitiesResponse, GetCampaignActivitiesError>;

    /// <p>Returns information about a specific version of a campaign.</p>
    fn get_campaign_version(
        &self,
        input: GetCampaignVersionRequest,
    ) -> RusotoFuture<GetCampaignVersionResponse, GetCampaignVersionError>;

    /// <p>Returns information about your campaign versions.</p>
    fn get_campaign_versions(
        &self,
        input: GetCampaignVersionsRequest,
    ) -> RusotoFuture<GetCampaignVersionsResponse, GetCampaignVersionsError>;

    /// <p>Returns information about your campaigns.</p>
    fn get_campaigns(
        &self,
        input: GetCampaignsRequest,
    ) -> RusotoFuture<GetCampaignsResponse, GetCampaignsError>;

    /// <p>Get all channels.</p>
    fn get_channels(
        &self,
        input: GetChannelsRequest,
    ) -> RusotoFuture<GetChannelsResponse, GetChannelsError>;

    /// <p>Get an email channel.</p>
    fn get_email_channel(
        &self,
        input: GetEmailChannelRequest,
    ) -> RusotoFuture<GetEmailChannelResponse, GetEmailChannelError>;

    /// <p>Returns information about an endpoint.</p>
    fn get_endpoint(
        &self,
        input: GetEndpointRequest,
    ) -> RusotoFuture<GetEndpointResponse, GetEndpointError>;

    /// <p>Returns the event stream for an app.</p>
    fn get_event_stream(
        &self,
        input: GetEventStreamRequest,
    ) -> RusotoFuture<GetEventStreamResponse, GetEventStreamError>;

    /// <p>Returns information about an export job.</p>
    fn get_export_job(
        &self,
        input: GetExportJobRequest,
    ) -> RusotoFuture<GetExportJobResponse, GetExportJobError>;

    /// <p>Returns information about your export jobs.</p>
    fn get_export_jobs(
        &self,
        input: GetExportJobsRequest,
    ) -> RusotoFuture<GetExportJobsResponse, GetExportJobsError>;

    /// <p>Returns information about the GCM channel for an app.</p>
    fn get_gcm_channel(
        &self,
        input: GetGcmChannelRequest,
    ) -> RusotoFuture<GetGcmChannelResponse, GetGcmChannelError>;

    /// <p>Returns information about an import job.</p>
    fn get_import_job(
        &self,
        input: GetImportJobRequest,
    ) -> RusotoFuture<GetImportJobResponse, GetImportJobError>;

    /// <p>Returns information about your import jobs.</p>
    fn get_import_jobs(
        &self,
        input: GetImportJobsRequest,
    ) -> RusotoFuture<GetImportJobsResponse, GetImportJobsError>;

    /// <p>Returns information about a segment.</p>
    fn get_segment(
        &self,
        input: GetSegmentRequest,
    ) -> RusotoFuture<GetSegmentResponse, GetSegmentError>;

    /// <p>Returns a list of export jobs for a specific segment.</p>
    fn get_segment_export_jobs(
        &self,
        input: GetSegmentExportJobsRequest,
    ) -> RusotoFuture<GetSegmentExportJobsResponse, GetSegmentExportJobsError>;

    /// <p>Returns a list of import jobs for a specific segment.</p>
    fn get_segment_import_jobs(
        &self,
        input: GetSegmentImportJobsRequest,
    ) -> RusotoFuture<GetSegmentImportJobsResponse, GetSegmentImportJobsError>;

    /// <p>Returns information about a segment version.</p>
    fn get_segment_version(
        &self,
        input: GetSegmentVersionRequest,
    ) -> RusotoFuture<GetSegmentVersionResponse, GetSegmentVersionError>;

    /// <p>Returns information about your segment versions.</p>
    fn get_segment_versions(
        &self,
        input: GetSegmentVersionsRequest,
    ) -> RusotoFuture<GetSegmentVersionsResponse, GetSegmentVersionsError>;

    /// <p>Used to get information about your segments.</p>
    fn get_segments(
        &self,
        input: GetSegmentsRequest,
    ) -> RusotoFuture<GetSegmentsResponse, GetSegmentsError>;

    /// <p>Get an SMS channel.</p>
    fn get_sms_channel(
        &self,
        input: GetSmsChannelRequest,
    ) -> RusotoFuture<GetSmsChannelResponse, GetSmsChannelError>;

    /// <p>Returns information about the endpoints associated with an user id.</p>
    fn get_user_endpoints(
        &self,
        input: GetUserEndpointsRequest,
    ) -> RusotoFuture<GetUserEndpointsResponse, GetUserEndpointsError>;

    /// <p>Returns information about the specified phone number.</p>
    fn phone_number_validate(
        &self,
        input: PhoneNumberValidateRequest,
    ) -> RusotoFuture<PhoneNumberValidateResponse, PhoneNumberValidateError>;

    /// <p>Use to create or update the event stream for an app.</p>
    fn put_event_stream(
        &self,
        input: PutEventStreamRequest,
    ) -> RusotoFuture<PutEventStreamResponse, PutEventStreamError>;

    /// <p>Used to remove the attributes for an app</p>
    fn remove_attributes(
        &self,
        input: RemoveAttributesRequest,
    ) -> RusotoFuture<RemoveAttributesResponse, RemoveAttributesError>;

    /// <p>Use this resource to send a direct message, which is a one time message that you send to a limited audience without creating a campaign. </p>
    ///
    /// <p>You can send the message to up to 100 recipients. You cannot use the message to engage a segment. When you send the message, Amazon Pinpoint delivers it immediately, and you cannot schedule the delivery. To engage a user segment, and to schedule the message delivery, create a campaign instead of sending a direct message.</p>
    ///
    /// <p>You can send a direct message as a push notification to your mobile app or as an SMS message to SMS-enabled devices.</p>
    fn send_messages(
        &self,
        input: SendMessagesRequest,
    ) -> RusotoFuture<SendMessagesResponse, SendMessagesError>;

    /// <p>Use this resource to message a list of users. Amazon Pinpoint sends the message to all of the endpoints that are associated with each user.</p>
    ///
    /// <p>A user represents an individual who is assigned a unique user ID, and this ID is assigned to one or more endpoints. For example, if an individual uses your app on multiple devices, your app could assign that person&#39;s user ID to the endpoint for each device.</p>
    ///
    /// <p>With the users-messages resource, you specify the message recipients as user IDs. For each user ID, Amazon Pinpoint delivers the message to all of the user&#39;s endpoints. Within the body of your request, you can specify a default message, and you can tailor your message for different channels, including those for mobile push and SMS.</p>
    ///
    /// <p>With this resource, you send a direct message, which is a one time message that you send to a limited audience without creating a campaign. You can send the message to up to 100 users per request. You cannot use the message to engage a segment. When you send the message, Amazon Pinpoint delivers it immediately, and you cannot schedule the delivery. To engage a user segment, and to schedule the message delivery, create a campaign instead of using the users-messages resource.</p>
    fn send_users_messages(
        &self,
        input: SendUsersMessagesRequest,
    ) -> RusotoFuture<SendUsersMessagesResponse, SendUsersMessagesError>;

    /// <p>Update an ADM channel.</p>
    fn update_adm_channel(
        &self,
        input: UpdateAdmChannelRequest,
    ) -> RusotoFuture<UpdateAdmChannelResponse, UpdateAdmChannelError>;

    /// <p>Use to update the APNs channel for an app.</p>
    fn update_apns_channel(
        &self,
        input: UpdateApnsChannelRequest,
    ) -> RusotoFuture<UpdateApnsChannelResponse, UpdateApnsChannelError>;

    /// <p>Update an APNS sandbox channel.</p>
    fn update_apns_sandbox_channel(
        &self,
        input: UpdateApnsSandboxChannelRequest,
    ) -> RusotoFuture<UpdateApnsSandboxChannelResponse, UpdateApnsSandboxChannelError>;

    /// <p>Update an APNS VoIP channel</p>
    fn update_apns_voip_channel(
        &self,
        input: UpdateApnsVoipChannelRequest,
    ) -> RusotoFuture<UpdateApnsVoipChannelResponse, UpdateApnsVoipChannelError>;

    /// <p>Update an APNS VoIP sandbox channel</p>
    fn update_apns_voip_sandbox_channel(
        &self,
        input: UpdateApnsVoipSandboxChannelRequest,
    ) -> RusotoFuture<UpdateApnsVoipSandboxChannelResponse, UpdateApnsVoipSandboxChannelError>;

    /// <p>Used to update the settings for an app.</p>
    fn update_application_settings(
        &self,
        input: UpdateApplicationSettingsRequest,
    ) -> RusotoFuture<UpdateApplicationSettingsResponse, UpdateApplicationSettingsError>;

    /// <p>Update a BAIDU GCM channel</p>
    fn update_baidu_channel(
        &self,
        input: UpdateBaiduChannelRequest,
    ) -> RusotoFuture<UpdateBaiduChannelResponse, UpdateBaiduChannelError>;

    /// <p>Use to update a campaign.</p>
    fn update_campaign(
        &self,
        input: UpdateCampaignRequest,
    ) -> RusotoFuture<UpdateCampaignResponse, UpdateCampaignError>;

    /// <p>Update an email channel.</p>
    fn update_email_channel(
        &self,
        input: UpdateEmailChannelRequest,
    ) -> RusotoFuture<UpdateEmailChannelResponse, UpdateEmailChannelError>;

    /// <p>Creates or updates an endpoint.</p>
    fn update_endpoint(
        &self,
        input: UpdateEndpointRequest,
    ) -> RusotoFuture<UpdateEndpointResponse, UpdateEndpointError>;

    /// <p>Use to update a batch of endpoints.</p>
    fn update_endpoints_batch(
        &self,
        input: UpdateEndpointsBatchRequest,
    ) -> RusotoFuture<UpdateEndpointsBatchResponse, UpdateEndpointsBatchError>;

    /// <p>Use to update the GCM channel for an app.</p>
    fn update_gcm_channel(
        &self,
        input: UpdateGcmChannelRequest,
    ) -> RusotoFuture<UpdateGcmChannelResponse, UpdateGcmChannelError>;

    /// <p>Use to update a segment.</p>
    fn update_segment(
        &self,
        input: UpdateSegmentRequest,
    ) -> RusotoFuture<UpdateSegmentResponse, UpdateSegmentError>;

    /// <p>Update an SMS channel.</p>
    fn update_sms_channel(
        &self,
        input: UpdateSmsChannelRequest,
    ) -> RusotoFuture<UpdateSmsChannelResponse, UpdateSmsChannelError>;
}
/// A client for the Amazon Pinpoint API.
pub struct PinpointClient {
    client: Client,
    region: region::Region,
}

impl PinpointClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> PinpointClient {
        PinpointClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> PinpointClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        PinpointClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Pinpoint for PinpointClient {
    /// <p>Creates or updates an app.</p>
    fn create_app(
        &self,
        input: CreateAppRequest,
    ) -> RusotoFuture<CreateAppResponse, CreateAppError> {
        let request_uri = "/v1/apps";

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.create_application_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result = serde_json::from_slice::<CreateAppResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateAppError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates or updates a campaign.</p>
    fn create_campaign(
        &self,
        input: CreateCampaignRequest,
    ) -> RusotoFuture<CreateCampaignResponse, CreateCampaignError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/campaigns",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_campaign_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<CreateCampaignResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateCampaignError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an export job.</p>
    fn create_export_job(
        &self,
        input: CreateExportJobRequest,
    ) -> RusotoFuture<CreateExportJobResponse, CreateExportJobError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/jobs/export",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.export_job_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<CreateExportJobResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateExportJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates or updates an import job.</p>
    fn create_import_job(
        &self,
        input: CreateImportJobRequest,
    ) -> RusotoFuture<CreateImportJobResponse, CreateImportJobError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/jobs/import",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.import_job_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<CreateImportJobResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateImportJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Used to create or update a segment.</p>
    fn create_segment(
        &self,
        input: CreateSegmentRequest,
    ) -> RusotoFuture<CreateSegmentResponse, CreateSegmentError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_segment_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<CreateSegmentResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateSegmentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Delete an ADM channel.</p>
    fn delete_adm_channel(
        &self,
        input: DeleteAdmChannelRequest,
    ) -> RusotoFuture<DeleteAdmChannelResponse, DeleteAdmChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/adm",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<DeleteAdmChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteAdmChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the APNs channel for an app.</p>
    fn delete_apns_channel(
        &self,
        input: DeleteApnsChannelRequest,
    ) -> RusotoFuture<DeleteApnsChannelResponse, DeleteApnsChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<DeleteApnsChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteApnsChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Delete an APNS sandbox channel.</p>
    fn delete_apns_sandbox_channel(
        &self,
        input: DeleteApnsSandboxChannelRequest,
    ) -> RusotoFuture<DeleteApnsSandboxChannelResponse, DeleteApnsSandboxChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_sandbox",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<DeleteApnsSandboxChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteApnsSandboxChannelError::from_response(response))
                }))
            }
        })
    }

    /// <p>Delete an APNS VoIP channel</p>
    fn delete_apns_voip_channel(
        &self,
        input: DeleteApnsVoipChannelRequest,
    ) -> RusotoFuture<DeleteApnsVoipChannelResponse, DeleteApnsVoipChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_voip",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<DeleteApnsVoipChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteApnsVoipChannelError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Delete an APNS VoIP sandbox channel</p>
    fn delete_apns_voip_sandbox_channel(
        &self,
        input: DeleteApnsVoipSandboxChannelRequest,
    ) -> RusotoFuture<DeleteApnsVoipSandboxChannelResponse, DeleteApnsVoipSandboxChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_voip_sandbox",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<DeleteApnsVoipSandboxChannelResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteApnsVoipSandboxChannelError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes an app.</p>
    fn delete_app(
        &self,
        input: DeleteAppRequest,
    ) -> RusotoFuture<DeleteAppResponse, DeleteAppError> {
        let request_uri = format!(
            "/v1/apps/{application_id}",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result = serde_json::from_slice::<DeleteAppResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteAppError::from_response(response))),
                )
            }
        })
    }

    /// <p>Delete a BAIDU GCM channel</p>
    fn delete_baidu_channel(
        &self,
        input: DeleteBaiduChannelRequest,
    ) -> RusotoFuture<DeleteBaiduChannelResponse, DeleteBaiduChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/baidu",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<DeleteBaiduChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteBaiduChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a campaign.</p>
    fn delete_campaign(
        &self,
        input: DeleteCampaignRequest,
    ) -> RusotoFuture<DeleteCampaignResponse, DeleteCampaignError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/campaigns/{campaign_id}",
            application_id = input.application_id,
            campaign_id = input.campaign_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<DeleteCampaignResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteCampaignError::from_response(response))),
                )
            }
        })
    }

    /// <p>Delete an email channel.</p>
    fn delete_email_channel(
        &self,
        input: DeleteEmailChannelRequest,
    ) -> RusotoFuture<DeleteEmailChannelResponse, DeleteEmailChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/email",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<DeleteEmailChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteEmailChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an endpoint.</p>
    fn delete_endpoint(
        &self,
        input: DeleteEndpointRequest,
    ) -> RusotoFuture<DeleteEndpointResponse, DeleteEndpointError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/endpoints/{endpoint_id}",
            application_id = input.application_id,
            endpoint_id = input.endpoint_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<DeleteEndpointResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the event stream for an app.</p>
    fn delete_event_stream(
        &self,
        input: DeleteEventStreamRequest,
    ) -> RusotoFuture<DeleteEventStreamResponse, DeleteEventStreamError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/eventstream",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<DeleteEventStreamResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteEventStreamError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the GCM channel for an app.</p>
    fn delete_gcm_channel(
        &self,
        input: DeleteGcmChannelRequest,
    ) -> RusotoFuture<DeleteGcmChannelResponse, DeleteGcmChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/gcm",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<DeleteGcmChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteGcmChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a segment.</p>
    fn delete_segment(
        &self,
        input: DeleteSegmentRequest,
    ) -> RusotoFuture<DeleteSegmentResponse, DeleteSegmentError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments/{segment_id}",
            application_id = input.application_id,
            segment_id = input.segment_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<DeleteSegmentResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteSegmentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Delete an SMS channel.</p>
    fn delete_sms_channel(
        &self,
        input: DeleteSmsChannelRequest,
    ) -> RusotoFuture<DeleteSmsChannelResponse, DeleteSmsChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/sms",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<DeleteSmsChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteSmsChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes endpoints associated with an user id.</p>
    fn delete_user_endpoints(
        &self,
        input: DeleteUserEndpointsRequest,
    ) -> RusotoFuture<DeleteUserEndpointsResponse, DeleteUserEndpointsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/users/{user_id}",
            application_id = input.application_id,
            user_id = input.user_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<DeleteUserEndpointsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteUserEndpointsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Get an ADM channel.</p>
    fn get_adm_channel(
        &self,
        input: GetAdmChannelRequest,
    ) -> RusotoFuture<GetAdmChannelResponse, GetAdmChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/adm",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<GetAdmChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAdmChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about the APNs channel for an app.</p>
    fn get_apns_channel(
        &self,
        input: GetApnsChannelRequest,
    ) -> RusotoFuture<GetApnsChannelResponse, GetApnsChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<GetApnsChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetApnsChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Get an APNS sandbox channel.</p>
    fn get_apns_sandbox_channel(
        &self,
        input: GetApnsSandboxChannelRequest,
    ) -> RusotoFuture<GetApnsSandboxChannelResponse, GetApnsSandboxChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_sandbox",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<GetApnsSandboxChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetApnsSandboxChannelError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Get an APNS VoIP channel</p>
    fn get_apns_voip_channel(
        &self,
        input: GetApnsVoipChannelRequest,
    ) -> RusotoFuture<GetApnsVoipChannelResponse, GetApnsVoipChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_voip",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<GetApnsVoipChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetApnsVoipChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Get an APNS VoIPSandbox channel</p>
    fn get_apns_voip_sandbox_channel(
        &self,
        input: GetApnsVoipSandboxChannelRequest,
    ) -> RusotoFuture<GetApnsVoipSandboxChannelResponse, GetApnsVoipSandboxChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_voip_sandbox",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<GetApnsVoipSandboxChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetApnsVoipSandboxChannelError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns information about an app.</p>
    fn get_app(&self, input: GetAppRequest) -> RusotoFuture<GetAppResponse, GetAppError> {
        let request_uri = format!(
            "/v1/apps/{application_id}",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result = serde_json::from_slice::<GetAppResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAppError::from_response(response))),
                )
            }
        })
    }

    /// <p>Used to request the settings for an app.</p>
    fn get_application_settings(
        &self,
        input: GetApplicationSettingsRequest,
    ) -> RusotoFuture<GetApplicationSettingsResponse, GetApplicationSettingsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/settings",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<GetApplicationSettingsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetApplicationSettingsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns information about your apps.</p>
    fn get_apps(&self, input: GetAppsRequest) -> RusotoFuture<GetAppsResponse, GetAppsError> {
        let request_uri = "/v1/apps";

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result = serde_json::from_slice::<GetAppsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAppsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Get a BAIDU GCM channel</p>
    fn get_baidu_channel(
        &self,
        input: GetBaiduChannelRequest,
    ) -> RusotoFuture<GetBaiduChannelResponse, GetBaiduChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/baidu",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<GetBaiduChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBaiduChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about a campaign.</p>
    fn get_campaign(
        &self,
        input: GetCampaignRequest,
    ) -> RusotoFuture<GetCampaignResponse, GetCampaignError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/campaigns/{campaign_id}",
            application_id = input.application_id,
            campaign_id = input.campaign_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result = serde_json::from_slice::<GetCampaignResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetCampaignError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about the activity performed by a campaign.</p>
    fn get_campaign_activities(
        &self,
        input: GetCampaignActivitiesRequest,
    ) -> RusotoFuture<GetCampaignActivitiesResponse, GetCampaignActivitiesError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/campaigns/{campaign_id}/activities",
            application_id = input.application_id,
            campaign_id = input.campaign_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<GetCampaignActivitiesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetCampaignActivitiesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns information about a specific version of a campaign.</p>
    fn get_campaign_version(
        &self,
        input: GetCampaignVersionRequest,
    ) -> RusotoFuture<GetCampaignVersionResponse, GetCampaignVersionError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/campaigns/{campaign_id}/versions/{version}",
            application_id = input.application_id,
            campaign_id = input.campaign_id,
            version = input.version
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<GetCampaignVersionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetCampaignVersionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about your campaign versions.</p>
    fn get_campaign_versions(
        &self,
        input: GetCampaignVersionsRequest,
    ) -> RusotoFuture<GetCampaignVersionsResponse, GetCampaignVersionsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/campaigns/{campaign_id}/versions",
            application_id = input.application_id,
            campaign_id = input.campaign_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<GetCampaignVersionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetCampaignVersionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns information about your campaigns.</p>
    fn get_campaigns(
        &self,
        input: GetCampaignsRequest,
    ) -> RusotoFuture<GetCampaignsResponse, GetCampaignsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/campaigns",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result = serde_json::from_slice::<GetCampaignsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetCampaignsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Get all channels.</p>
    fn get_channels(
        &self,
        input: GetChannelsRequest,
    ) -> RusotoFuture<GetChannelsResponse, GetChannelsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result = serde_json::from_slice::<GetChannelsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetChannelsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Get an email channel.</p>
    fn get_email_channel(
        &self,
        input: GetEmailChannelRequest,
    ) -> RusotoFuture<GetEmailChannelResponse, GetEmailChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/email",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<GetEmailChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetEmailChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about an endpoint.</p>
    fn get_endpoint(
        &self,
        input: GetEndpointRequest,
    ) -> RusotoFuture<GetEndpointResponse, GetEndpointError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/endpoints/{endpoint_id}",
            application_id = input.application_id,
            endpoint_id = input.endpoint_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result = serde_json::from_slice::<GetEndpointResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the event stream for an app.</p>
    fn get_event_stream(
        &self,
        input: GetEventStreamRequest,
    ) -> RusotoFuture<GetEventStreamResponse, GetEventStreamError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/eventstream",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<GetEventStreamResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetEventStreamError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about an export job.</p>
    fn get_export_job(
        &self,
        input: GetExportJobRequest,
    ) -> RusotoFuture<GetExportJobResponse, GetExportJobError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/jobs/export/{job_id}",
            application_id = input.application_id,
            job_id = input.job_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result = serde_json::from_slice::<GetExportJobResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetExportJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about your export jobs.</p>
    fn get_export_jobs(
        &self,
        input: GetExportJobsRequest,
    ) -> RusotoFuture<GetExportJobsResponse, GetExportJobsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/jobs/export",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<GetExportJobsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetExportJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about the GCM channel for an app.</p>
    fn get_gcm_channel(
        &self,
        input: GetGcmChannelRequest,
    ) -> RusotoFuture<GetGcmChannelResponse, GetGcmChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/gcm",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<GetGcmChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetGcmChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about an import job.</p>
    fn get_import_job(
        &self,
        input: GetImportJobRequest,
    ) -> RusotoFuture<GetImportJobResponse, GetImportJobError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/jobs/import/{job_id}",
            application_id = input.application_id,
            job_id = input.job_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result = serde_json::from_slice::<GetImportJobResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetImportJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about your import jobs.</p>
    fn get_import_jobs(
        &self,
        input: GetImportJobsRequest,
    ) -> RusotoFuture<GetImportJobsResponse, GetImportJobsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/jobs/import",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<GetImportJobsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetImportJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about a segment.</p>
    fn get_segment(
        &self,
        input: GetSegmentRequest,
    ) -> RusotoFuture<GetSegmentResponse, GetSegmentError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments/{segment_id}",
            application_id = input.application_id,
            segment_id = input.segment_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result = serde_json::from_slice::<GetSegmentResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSegmentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of export jobs for a specific segment.</p>
    fn get_segment_export_jobs(
        &self,
        input: GetSegmentExportJobsRequest,
    ) -> RusotoFuture<GetSegmentExportJobsResponse, GetSegmentExportJobsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments/{segment_id}/jobs/export",
            application_id = input.application_id,
            segment_id = input.segment_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<GetSegmentExportJobsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetSegmentExportJobsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns a list of import jobs for a specific segment.</p>
    fn get_segment_import_jobs(
        &self,
        input: GetSegmentImportJobsRequest,
    ) -> RusotoFuture<GetSegmentImportJobsResponse, GetSegmentImportJobsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments/{segment_id}/jobs/import",
            application_id = input.application_id,
            segment_id = input.segment_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<GetSegmentImportJobsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetSegmentImportJobsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns information about a segment version.</p>
    fn get_segment_version(
        &self,
        input: GetSegmentVersionRequest,
    ) -> RusotoFuture<GetSegmentVersionResponse, GetSegmentVersionError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments/{segment_id}/versions/{version}",
            application_id = input.application_id,
            segment_id = input.segment_id,
            version = input.version
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<GetSegmentVersionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSegmentVersionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about your segment versions.</p>
    fn get_segment_versions(
        &self,
        input: GetSegmentVersionsRequest,
    ) -> RusotoFuture<GetSegmentVersionsResponse, GetSegmentVersionsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments/{segment_id}/versions",
            application_id = input.application_id,
            segment_id = input.segment_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<GetSegmentVersionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSegmentVersionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Used to get information about your segments.</p>
    fn get_segments(
        &self,
        input: GetSegmentsRequest,
    ) -> RusotoFuture<GetSegmentsResponse, GetSegmentsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result = serde_json::from_slice::<GetSegmentsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSegmentsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Get an SMS channel.</p>
    fn get_sms_channel(
        &self,
        input: GetSmsChannelRequest,
    ) -> RusotoFuture<GetSmsChannelResponse, GetSmsChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/sms",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<GetSmsChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSmsChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about the endpoints associated with an user id.</p>
    fn get_user_endpoints(
        &self,
        input: GetUserEndpointsRequest,
    ) -> RusotoFuture<GetUserEndpointsResponse, GetUserEndpointsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/users/{user_id}",
            application_id = input.application_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<GetUserEndpointsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetUserEndpointsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about the specified phone number.</p>
    fn phone_number_validate(
        &self,
        input: PhoneNumberValidateRequest,
    ) -> RusotoFuture<PhoneNumberValidateResponse, PhoneNumberValidateError> {
        let request_uri = "/v1/phone/number/validate";

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.number_validate_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<PhoneNumberValidateResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PhoneNumberValidateError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Use to create or update the event stream for an app.</p>
    fn put_event_stream(
        &self,
        input: PutEventStreamRequest,
    ) -> RusotoFuture<PutEventStreamResponse, PutEventStreamError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/eventstream",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_event_stream).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<PutEventStreamResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutEventStreamError::from_response(response))),
                )
            }
        })
    }

    /// <p>Used to remove the attributes for an app</p>
    fn remove_attributes(
        &self,
        input: RemoveAttributesRequest,
    ) -> RusotoFuture<RemoveAttributesResponse, RemoveAttributesError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/attributes/{attribute_type}",
            application_id = input.application_id,
            attribute_type = input.attribute_type
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.update_attributes_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<RemoveAttributesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RemoveAttributesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Use this resource to send a direct message, which is a one time message that you send to a limited audience without creating a campaign. </p>
    ///
    /// <p>You can send the message to up to 100 recipients. You cannot use the message to engage a segment. When you send the message, Amazon Pinpoint delivers it immediately, and you cannot schedule the delivery. To engage a user segment, and to schedule the message delivery, create a campaign instead of sending a direct message.</p>
    ///
    /// <p>You can send a direct message as a push notification to your mobile app or as an SMS message to SMS-enabled devices.</p>
    fn send_messages(
        &self,
        input: SendMessagesRequest,
    ) -> RusotoFuture<SendMessagesResponse, SendMessagesError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/messages",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.message_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result = serde_json::from_slice::<SendMessagesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SendMessagesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Use this resource to message a list of users. Amazon Pinpoint sends the message to all of the endpoints that are associated with each user.</p>
    ///
    /// <p>A user represents an individual who is assigned a unique user ID, and this ID is assigned to one or more endpoints. For example, if an individual uses your app on multiple devices, your app could assign that person&#39;s user ID to the endpoint for each device.</p>
    ///
    /// <p>With the users-messages resource, you specify the message recipients as user IDs. For each user ID, Amazon Pinpoint delivers the message to all of the user&#39;s endpoints. Within the body of your request, you can specify a default message, and you can tailor your message for different channels, including those for mobile push and SMS.</p>
    ///
    /// <p>With this resource, you send a direct message, which is a one time message that you send to a limited audience without creating a campaign. You can send the message to up to 100 users per request. You cannot use the message to engage a segment. When you send the message, Amazon Pinpoint delivers it immediately, and you cannot schedule the delivery. To engage a user segment, and to schedule the message delivery, create a campaign instead of using the users-messages resource.</p>
    fn send_users_messages(
        &self,
        input: SendUsersMessagesRequest,
    ) -> RusotoFuture<SendUsersMessagesResponse, SendUsersMessagesError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/users-messages",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.send_users_message_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<SendUsersMessagesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SendUsersMessagesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Update an ADM channel.</p>
    fn update_adm_channel(
        &self,
        input: UpdateAdmChannelRequest,
    ) -> RusotoFuture<UpdateAdmChannelResponse, UpdateAdmChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/adm",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.adm_channel_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<UpdateAdmChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateAdmChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Use to update the APNs channel for an app.</p>
    fn update_apns_channel(
        &self,
        input: UpdateApnsChannelRequest,
    ) -> RusotoFuture<UpdateApnsChannelResponse, UpdateApnsChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.apns_channel_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<UpdateApnsChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateApnsChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Update an APNS sandbox channel.</p>
    fn update_apns_sandbox_channel(
        &self,
        input: UpdateApnsSandboxChannelRequest,
    ) -> RusotoFuture<UpdateApnsSandboxChannelResponse, UpdateApnsSandboxChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_sandbox",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.apns_sandbox_channel_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<UpdateApnsSandboxChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateApnsSandboxChannelError::from_response(response))
                }))
            }
        })
    }

    /// <p>Update an APNS VoIP channel</p>
    fn update_apns_voip_channel(
        &self,
        input: UpdateApnsVoipChannelRequest,
    ) -> RusotoFuture<UpdateApnsVoipChannelResponse, UpdateApnsVoipChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_voip",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.apns_voip_channel_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<UpdateApnsVoipChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateApnsVoipChannelError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Update an APNS VoIP sandbox channel</p>
    fn update_apns_voip_sandbox_channel(
        &self,
        input: UpdateApnsVoipSandboxChannelRequest,
    ) -> RusotoFuture<UpdateApnsVoipSandboxChannelResponse, UpdateApnsVoipSandboxChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_voip_sandbox",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.apns_voip_sandbox_channel_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<UpdateApnsVoipSandboxChannelResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateApnsVoipSandboxChannelError::from_response(response))
                }))
            }
        })
    }

    /// <p>Used to update the settings for an app.</p>
    fn update_application_settings(
        &self,
        input: UpdateApplicationSettingsRequest,
    ) -> RusotoFuture<UpdateApplicationSettingsResponse, UpdateApplicationSettingsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/settings",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_application_settings_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<UpdateApplicationSettingsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateApplicationSettingsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Update a BAIDU GCM channel</p>
    fn update_baidu_channel(
        &self,
        input: UpdateBaiduChannelRequest,
    ) -> RusotoFuture<UpdateBaiduChannelResponse, UpdateBaiduChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/baidu",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.baidu_channel_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<UpdateBaiduChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateBaiduChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Use to update a campaign.</p>
    fn update_campaign(
        &self,
        input: UpdateCampaignRequest,
    ) -> RusotoFuture<UpdateCampaignResponse, UpdateCampaignError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/campaigns/{campaign_id}",
            application_id = input.application_id,
            campaign_id = input.campaign_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_campaign_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<UpdateCampaignResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateCampaignError::from_response(response))),
                )
            }
        })
    }

    /// <p>Update an email channel.</p>
    fn update_email_channel(
        &self,
        input: UpdateEmailChannelRequest,
    ) -> RusotoFuture<UpdateEmailChannelResponse, UpdateEmailChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/email",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.email_channel_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<UpdateEmailChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateEmailChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates or updates an endpoint.</p>
    fn update_endpoint(
        &self,
        input: UpdateEndpointRequest,
    ) -> RusotoFuture<UpdateEndpointResponse, UpdateEndpointError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/endpoints/{endpoint_id}",
            application_id = input.application_id,
            endpoint_id = input.endpoint_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.endpoint_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<UpdateEndpointResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p>Use to update a batch of endpoints.</p>
    fn update_endpoints_batch(
        &self,
        input: UpdateEndpointsBatchRequest,
    ) -> RusotoFuture<UpdateEndpointsBatchResponse, UpdateEndpointsBatchError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/endpoints",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.endpoint_batch_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<UpdateEndpointsBatchResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateEndpointsBatchError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Use to update the GCM channel for an app.</p>
    fn update_gcm_channel(
        &self,
        input: UpdateGcmChannelRequest,
    ) -> RusotoFuture<UpdateGcmChannelResponse, UpdateGcmChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/gcm",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.gcm_channel_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<UpdateGcmChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateGcmChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Use to update a segment.</p>
    fn update_segment(
        &self,
        input: UpdateSegmentRequest,
    ) -> RusotoFuture<UpdateSegmentResponse, UpdateSegmentError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments/{segment_id}",
            application_id = input.application_id,
            segment_id = input.segment_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_segment_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<UpdateSegmentResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateSegmentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Update an SMS channel.</p>
    fn update_sms_channel(
        &self,
        input: UpdateSmsChannelRequest,
    ) -> RusotoFuture<UpdateSmsChannelResponse, UpdateSmsChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/sms",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.sms_channel_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result =
                        serde_json::from_slice::<UpdateSmsChannelResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateSmsChannelError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
