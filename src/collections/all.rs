//! Enums for heterogeneous collections of events, inclusive for every event type that implements
//! the trait of the same name.

use serde::{Serialize, Serializer};

use super::raw::all as raw;
use crate::{
    call::{
        answer::AnswerEvent, candidates::CandidatesEvent, hangup::HangupEvent, invite::InviteEvent,
    },
    direct::DirectEvent,
    dummy::DummyEvent,
    forwarded_room_key::ForwardedRoomKeyEvent,
    fully_read::FullyReadEvent,
    ignored_user_list::IgnoredUserListEvent,
    key::verification::{
        accept::AcceptEvent, cancel::CancelEvent, key::KeyEvent, mac::MacEvent,
        request::RequestEvent, start::StartEvent,
    },
    presence::PresenceEvent,
    push_rules::PushRulesEvent,
    receipt::ReceiptEvent,
    room::{
        aliases::AliasesEvent,
        avatar::AvatarEvent,
        canonical_alias::CanonicalAliasEvent,
        create::CreateEvent,
        encrypted::EncryptedEvent,
        encryption::EncryptionEvent,
        guest_access::GuestAccessEvent,
        history_visibility::HistoryVisibilityEvent,
        join_rules::JoinRulesEvent,
        member::MemberEvent,
        message::{feedback::FeedbackEvent, MessageEvent},
        name::NameEvent,
        pinned_events::PinnedEventsEvent,
        power_levels::PowerLevelsEvent,
        redaction::RedactionEvent,
        server_acl::ServerAclEvent,
        third_party_invite::ThirdPartyInviteEvent,
        tombstone::TombstoneEvent,
        topic::TopicEvent,
    },
    room_key::RoomKeyEvent,
    room_key_request::RoomKeyRequestEvent,
    sticker::StickerEvent,
    tag::TagEvent,
    typing::TypingEvent,
    CustomEvent, CustomRoomEvent, CustomStateEvent, TryFromRaw,
};

/// A basic event, room event, or state event.
#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum Event {
    /// m.call.answer
    CallAnswer(AnswerEvent),

    /// m.call.candidates
    CallCandidates(CandidatesEvent),

    /// m.call.hangup
    CallHangup(HangupEvent),

    /// m.call.invite
    CallInvite(InviteEvent),

    /// m.direct
    Direct(DirectEvent),

    /// m.dummy
    Dummy(DummyEvent),

    /// m.forwarded_room_key
    ForwardedRoomKey(ForwardedRoomKeyEvent),

    /// m.fully_read
    FullyRead(FullyReadEvent),

    /// m.ignored_user_list
    IgnoredUserList(IgnoredUserListEvent),

    /// m.key.verification.accept
    KeyVerificationAccept(AcceptEvent),

    /// m.key.verification.cancel
    KeyVerificationCancel(CancelEvent),

    /// m.key.verification.key
    KeyVerificationKey(KeyEvent),

    /// m.key.verification.mac
    KeyVerificationMac(MacEvent),

    /// m.key.verification.request
    KeyVerificationRequest(RequestEvent),

    /// m.key.verification.start
    KeyVerificationStart(StartEvent),

    /// m.presence
    Presence(PresenceEvent),

    /// m.push_rules
    PushRules(PushRulesEvent),

    /// m.receipt
    Receipt(ReceiptEvent),

    /// m.room.aliases
    RoomAliases(AliasesEvent),

    /// m.room.avatar
    RoomAvatar(AvatarEvent),

    /// m.room.canonical_alias
    RoomCanonicalAlias(CanonicalAliasEvent),

    /// m.room.create
    RoomCreate(CreateEvent),

    /// m.room.encrypted
    RoomEncrypted(EncryptedEvent),

    /// m.room.encryption
    RoomEncryption(EncryptionEvent),

    /// m.room.guest_access
    RoomGuestAccess(GuestAccessEvent),

    /// m.room.history_visibility
    RoomHistoryVisibility(HistoryVisibilityEvent),

    /// m.room.join_rules
    RoomJoinRules(JoinRulesEvent),

    /// m.room.member
    RoomMember(MemberEvent),

    /// m.room.message
    RoomMessage(MessageEvent),

    /// m.room.message.feedback
    RoomMessageFeedback(FeedbackEvent),

    /// m.room.name
    RoomName(NameEvent),

    /// m.room.pinned_events
    RoomPinnedEvents(PinnedEventsEvent),

    /// m.room.power_levels
    RoomPowerLevels(PowerLevelsEvent),

    /// m.room.redaction
    RoomRedaction(RedactionEvent),

    /// m.room.server_acl
    RoomServerAcl(ServerAclEvent),

    /// m.room.third_party_invite
    RoomThirdPartyInvite(ThirdPartyInviteEvent),

    /// m.room.tombstone
    RoomTombstone(TombstoneEvent),

    /// m.room.topic
    RoomTopic(TopicEvent),

    /// m.room_key
    RoomKey(RoomKeyEvent),

    /// m.room_key_request
    RoomKeyRequest(RoomKeyRequestEvent),

    /// m.sticker
    Sticker(StickerEvent),

    /// m.tag
    Tag(TagEvent),

    /// m.typing
    Typing(TypingEvent),

    /// Any basic event that is not part of the specification.
    Custom(CustomEvent),

    /// Any room event that is not part of the specification.
    CustomRoom(CustomRoomEvent),

    /// Any state event that is not part of the specification.
    CustomState(CustomStateEvent),
}

/// A room event or state event.
#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum RoomEvent {
    /// m.call.answer
    CallAnswer(AnswerEvent),

    /// m.call.candidates
    CallCandidates(CandidatesEvent),

    /// m.call.hangup
    CallHangup(HangupEvent),

    /// m.call.invite
    CallInvite(InviteEvent),

    /// m.room.aliases
    RoomAliases(AliasesEvent),

    /// m.room.avatar
    RoomAvatar(AvatarEvent),

    /// m.room.canonical_alias
    RoomCanonicalAlias(CanonicalAliasEvent),

    /// m.room.create
    RoomCreate(CreateEvent),

    /// m.room.encrypted
    RoomEncrypted(EncryptedEvent),

    /// m.room.encryption
    RoomEncryption(EncryptionEvent),

    /// m.room.guest_access
    RoomGuestAccess(GuestAccessEvent),

    /// m.room.history_visibility
    RoomHistoryVisibility(HistoryVisibilityEvent),

    /// m.room.join_rules
    RoomJoinRules(JoinRulesEvent),

    /// m.room.member
    RoomMember(MemberEvent),

    /// m.room.message
    RoomMessage(MessageEvent),

    /// m.room.message.feedback
    RoomMessageFeedback(FeedbackEvent),

    /// m.room.name
    RoomName(NameEvent),

    /// m.room.pinned_events
    RoomPinnedEvents(PinnedEventsEvent),

    /// m.room.power_levels
    RoomPowerLevels(PowerLevelsEvent),

    /// m.room.redaction
    RoomRedaction(RedactionEvent),

    /// m.room.server_acl
    RoomServerAcl(ServerAclEvent),

    /// m.room.third_party_invite
    RoomThirdPartyInvite(ThirdPartyInviteEvent),

    /// m.room.tombstone
    RoomTombstone(TombstoneEvent),

    /// m.room.topic
    RoomTopic(TopicEvent),

    /// m.sticker
    Sticker(StickerEvent),

    /// Any room event that is not part of the specification.
    CustomRoom(CustomRoomEvent),

    /// Any state event that is not part of the specification.
    CustomState(CustomStateEvent),
}

/// A state event.
#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum StateEvent {
    /// m.room.aliases
    RoomAliases(AliasesEvent),

    /// m.room.avatar
    RoomAvatar(AvatarEvent),

    /// m.room.canonical_alias
    RoomCanonicalAlias(CanonicalAliasEvent),

    /// m.room.create
    RoomCreate(CreateEvent),

    /// m.room.encryption
    RoomEncryption(EncryptionEvent),

    /// m.room.guest_access
    RoomGuestAccess(GuestAccessEvent),

    /// m.room.history_visibility
    RoomHistoryVisibility(HistoryVisibilityEvent),

    /// m.room.join_rules
    RoomJoinRules(JoinRulesEvent),

    /// m.room.member
    RoomMember(MemberEvent),

    /// m.room.name
    RoomName(NameEvent),

    /// m.room.pinned_events
    RoomPinnedEvents(PinnedEventsEvent),

    /// m.room.power_levels
    RoomPowerLevels(PowerLevelsEvent),

    /// m.room.server_acl
    RoomServerAcl(ServerAclEvent),

    /// m.room.third_party_invite
    RoomThirdPartyInvite(ThirdPartyInviteEvent),

    /// m.room.tombstone
    RoomTombstone(TombstoneEvent),

    /// m.room.topic
    RoomTopic(TopicEvent),

    /// Any state event that is not part of the specification.
    CustomState(CustomStateEvent),
}

impl TryFromRaw for Event {
    type Raw = raw::Event;
    type Err = String;

    fn try_from_raw(raw: raw::Event) -> Result<Self, (Self::Err, Self::Raw)> {
        use crate::try_convert_variant as conv;
        use raw::Event::*;

        match raw {
            CallAnswer(c) => conv(CallAnswer, Self::CallAnswer, c),
            CallCandidates(c) => conv(CallCandidates, Self::CallCandidates, c),
            CallHangup(c) => conv(CallHangup, Self::CallHangup, c),
            CallInvite(c) => conv(CallInvite, Self::CallInvite, c),
            Direct(c) => conv(Direct, Self::Direct, c),
            Dummy(c) => conv(Dummy, Self::Dummy, c),
            ForwardedRoomKey(c) => conv(ForwardedRoomKey, Self::ForwardedRoomKey, c),
            FullyRead(c) => conv(FullyRead, Self::FullyRead, c),
            IgnoredUserList(c) => conv(IgnoredUserList, Self::IgnoredUserList, c),
            KeyVerificationAccept(c) => conv(KeyVerificationAccept, Self::KeyVerificationAccept, c),
            KeyVerificationCancel(c) => conv(KeyVerificationCancel, Self::KeyVerificationCancel, c),
            KeyVerificationKey(c) => conv(KeyVerificationKey, Self::KeyVerificationKey, c),
            KeyVerificationMac(c) => conv(KeyVerificationMac, Self::KeyVerificationMac, c),
            KeyVerificationRequest(c) => {
                conv(KeyVerificationRequest, Self::KeyVerificationRequest, c)
            }
            KeyVerificationStart(c) => conv(KeyVerificationStart, Self::KeyVerificationStart, c),
            Presence(c) => conv(Presence, Self::Presence, c),
            PushRules(c) => conv(PushRules, Self::PushRules, c),
            Receipt(c) => conv(Receipt, Self::Receipt, c),
            RoomAliases(c) => conv(RoomAliases, Self::RoomAliases, c),
            RoomAvatar(c) => conv(RoomAvatar, Self::RoomAvatar, c),
            RoomCanonicalAlias(c) => conv(RoomCanonicalAlias, Self::RoomCanonicalAlias, c),
            RoomCreate(c) => conv(RoomCreate, Self::RoomCreate, c),
            RoomEncrypted(c) => conv(RoomEncrypted, Self::RoomEncrypted, c),
            RoomEncryption(c) => conv(RoomEncryption, Self::RoomEncryption, c),
            RoomGuestAccess(c) => conv(RoomGuestAccess, Self::RoomGuestAccess, c),
            RoomHistoryVisibility(c) => conv(RoomHistoryVisibility, Self::RoomHistoryVisibility, c),
            RoomJoinRules(c) => conv(RoomJoinRules, Self::RoomJoinRules, c),
            RoomMember(c) => conv(RoomMember, Self::RoomMember, c),
            RoomMessage(c) => conv(RoomMessage, Self::RoomMessage, c),
            RoomMessageFeedback(c) => conv(RoomMessageFeedback, Self::RoomMessageFeedback, c),
            RoomName(c) => conv(RoomName, Self::RoomName, c),
            RoomPinnedEvents(c) => conv(RoomPinnedEvents, Self::RoomPinnedEvents, c),
            RoomPowerLevels(c) => conv(RoomPowerLevels, Self::RoomPowerLevels, c),
            RoomRedaction(c) => conv(RoomRedaction, Self::RoomRedaction, c),
            RoomServerAcl(c) => conv(RoomServerAcl, Self::RoomServerAcl, c),
            RoomThirdPartyInvite(c) => conv(RoomThirdPartyInvite, Self::RoomThirdPartyInvite, c),
            RoomTombstone(c) => conv(RoomTombstone, Self::RoomTombstone, c),
            RoomTopic(c) => conv(RoomTopic, Self::RoomTopic, c),
            RoomKey(c) => conv(RoomKey, Self::RoomKey, c),
            RoomKeyRequest(c) => conv(RoomKeyRequest, Self::RoomKeyRequest, c),
            Sticker(c) => conv(Sticker, Self::Sticker, c),
            Tag(c) => conv(Tag, Self::Tag, c),
            Typing(c) => conv(Typing, Self::Typing, c),
            Custom(c) => Ok(Self::Custom(c)),
            CustomRoom(c) => Ok(Self::CustomRoom(c)),
            CustomState(c) => Ok(Self::CustomState(c)),
        }
    }
}

impl TryFromRaw for RoomEvent {
    type Raw = raw::RoomEvent;
    type Err = String;

    fn try_from_raw(raw: raw::RoomEvent) -> Result<Self, (Self::Err, Self::Raw)> {
        use crate::try_convert_variant as conv;
        use raw::RoomEvent::*;

        match raw {
            CallAnswer(c) => conv(CallAnswer, Self::CallAnswer, c),
            CallCandidates(c) => conv(CallCandidates, Self::CallCandidates, c),
            CallHangup(c) => conv(CallHangup, Self::CallHangup, c),
            CallInvite(c) => conv(CallInvite, Self::CallInvite, c),
            RoomAliases(c) => conv(RoomAliases, Self::RoomAliases, c),
            RoomAvatar(c) => conv(RoomAvatar, Self::RoomAvatar, c),
            RoomCanonicalAlias(c) => conv(RoomCanonicalAlias, Self::RoomCanonicalAlias, c),
            RoomCreate(c) => conv(RoomCreate, Self::RoomCreate, c),
            RoomEncrypted(c) => conv(RoomEncrypted, Self::RoomEncrypted, c),
            RoomEncryption(c) => conv(RoomEncryption, Self::RoomEncryption, c),
            RoomGuestAccess(c) => conv(RoomGuestAccess, Self::RoomGuestAccess, c),
            RoomHistoryVisibility(c) => conv(RoomHistoryVisibility, Self::RoomHistoryVisibility, c),
            RoomJoinRules(c) => conv(RoomJoinRules, Self::RoomJoinRules, c),
            RoomMember(c) => conv(RoomMember, Self::RoomMember, c),
            RoomMessage(c) => conv(RoomMessage, Self::RoomMessage, c),
            RoomMessageFeedback(c) => conv(RoomMessageFeedback, Self::RoomMessageFeedback, c),
            RoomName(c) => conv(RoomName, Self::RoomName, c),
            RoomPinnedEvents(c) => conv(RoomPinnedEvents, Self::RoomPinnedEvents, c),
            RoomPowerLevels(c) => conv(RoomPowerLevels, Self::RoomPowerLevels, c),
            RoomRedaction(c) => conv(RoomRedaction, Self::RoomRedaction, c),
            RoomServerAcl(c) => conv(RoomServerAcl, Self::RoomServerAcl, c),
            RoomThirdPartyInvite(c) => conv(RoomThirdPartyInvite, Self::RoomThirdPartyInvite, c),
            RoomTombstone(c) => conv(RoomTombstone, Self::RoomTombstone, c),
            RoomTopic(c) => conv(RoomTopic, Self::RoomTopic, c),
            Sticker(c) => conv(Sticker, Self::Sticker, c),
            CustomRoom(c) => Ok(Self::CustomRoom(c)),
            CustomState(c) => Ok(Self::CustomState(c)),
        }
    }
}

impl TryFromRaw for StateEvent {
    type Raw = raw::StateEvent;
    type Err = String;

    fn try_from_raw(raw: raw::StateEvent) -> Result<Self, (Self::Err, Self::Raw)> {
        use crate::try_convert_variant as conv;
        use raw::StateEvent::*;

        match raw {
            RoomAliases(c) => conv(RoomAliases, Self::RoomAliases, c),
            RoomAvatar(c) => conv(RoomAvatar, Self::RoomAvatar, c),
            RoomCanonicalAlias(c) => conv(RoomCanonicalAlias, Self::RoomCanonicalAlias, c),
            RoomCreate(c) => conv(RoomCreate, Self::RoomCreate, c),
            RoomEncryption(c) => conv(RoomEncryption, Self::RoomEncryption, c),
            RoomGuestAccess(c) => conv(RoomGuestAccess, Self::RoomGuestAccess, c),
            RoomHistoryVisibility(c) => conv(RoomHistoryVisibility, Self::RoomHistoryVisibility, c),
            RoomJoinRules(c) => conv(RoomJoinRules, Self::RoomJoinRules, c),
            RoomMember(c) => conv(RoomMember, Self::RoomMember, c),
            RoomName(c) => conv(RoomName, Self::RoomName, c),
            RoomPinnedEvents(c) => conv(RoomPinnedEvents, Self::RoomPinnedEvents, c),
            RoomPowerLevels(c) => conv(RoomPowerLevels, Self::RoomPowerLevels, c),
            RoomServerAcl(c) => conv(RoomServerAcl, Self::RoomServerAcl, c),
            RoomThirdPartyInvite(c) => conv(RoomThirdPartyInvite, Self::RoomThirdPartyInvite, c),
            RoomTombstone(c) => conv(RoomTombstone, Self::RoomTombstone, c),
            RoomTopic(c) => conv(RoomTopic, Self::RoomTopic, c),
            CustomState(c) => Ok(Self::CustomState(c)),
        }
    }
}

impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Event::CallAnswer(ref event) => event.serialize(serializer),
            Event::CallCandidates(ref event) => event.serialize(serializer),
            Event::CallHangup(ref event) => event.serialize(serializer),
            Event::CallInvite(ref event) => event.serialize(serializer),
            Event::Direct(ref event) => event.serialize(serializer),
            Event::Dummy(ref event) => event.serialize(serializer),
            Event::ForwardedRoomKey(ref event) => event.serialize(serializer),
            Event::FullyRead(ref event) => event.serialize(serializer),
            Event::KeyVerificationAccept(ref event) => event.serialize(serializer),
            Event::KeyVerificationCancel(ref event) => event.serialize(serializer),
            Event::KeyVerificationKey(ref event) => event.serialize(serializer),
            Event::KeyVerificationMac(ref event) => event.serialize(serializer),
            Event::KeyVerificationRequest(ref event) => event.serialize(serializer),
            Event::KeyVerificationStart(ref event) => event.serialize(serializer),
            Event::IgnoredUserList(ref event) => event.serialize(serializer),
            Event::Presence(ref event) => event.serialize(serializer),
            Event::PushRules(ref event) => event.serialize(serializer),
            Event::Receipt(ref event) => event.serialize(serializer),
            Event::RoomAliases(ref event) => event.serialize(serializer),
            Event::RoomAvatar(ref event) => event.serialize(serializer),
            Event::RoomCanonicalAlias(ref event) => event.serialize(serializer),
            Event::RoomCreate(ref event) => event.serialize(serializer),
            Event::RoomEncrypted(ref event) => event.serialize(serializer),
            Event::RoomEncryption(ref event) => event.serialize(serializer),
            Event::RoomGuestAccess(ref event) => event.serialize(serializer),
            Event::RoomHistoryVisibility(ref event) => event.serialize(serializer),
            Event::RoomJoinRules(ref event) => event.serialize(serializer),
            Event::RoomMember(ref event) => event.serialize(serializer),
            Event::RoomMessage(ref event) => event.serialize(serializer),
            Event::RoomMessageFeedback(ref event) => event.serialize(serializer),
            Event::RoomName(ref event) => event.serialize(serializer),
            Event::RoomPinnedEvents(ref event) => event.serialize(serializer),
            Event::RoomPowerLevels(ref event) => event.serialize(serializer),
            Event::RoomRedaction(ref event) => event.serialize(serializer),
            Event::RoomServerAcl(ref event) => event.serialize(serializer),
            Event::RoomThirdPartyInvite(ref event) => event.serialize(serializer),
            Event::RoomTombstone(ref event) => event.serialize(serializer),
            Event::RoomTopic(ref event) => event.serialize(serializer),
            Event::RoomKey(ref event) => event.serialize(serializer),
            Event::RoomKeyRequest(ref event) => event.serialize(serializer),
            Event::Sticker(ref event) => event.serialize(serializer),
            Event::Tag(ref event) => event.serialize(serializer),
            Event::Typing(ref event) => event.serialize(serializer),
            Event::Custom(ref event) => event.serialize(serializer),
            Event::CustomRoom(ref event) => event.serialize(serializer),
            Event::CustomState(ref event) => event.serialize(serializer),
        }
    }
}

impl Serialize for RoomEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            RoomEvent::CallAnswer(ref event) => event.serialize(serializer),
            RoomEvent::CallCandidates(ref event) => event.serialize(serializer),
            RoomEvent::CallHangup(ref event) => event.serialize(serializer),
            RoomEvent::CallInvite(ref event) => event.serialize(serializer),
            RoomEvent::RoomAliases(ref event) => event.serialize(serializer),
            RoomEvent::RoomAvatar(ref event) => event.serialize(serializer),
            RoomEvent::RoomCanonicalAlias(ref event) => event.serialize(serializer),
            RoomEvent::RoomCreate(ref event) => event.serialize(serializer),
            RoomEvent::RoomEncrypted(ref event) => event.serialize(serializer),
            RoomEvent::RoomEncryption(ref event) => event.serialize(serializer),
            RoomEvent::RoomGuestAccess(ref event) => event.serialize(serializer),
            RoomEvent::RoomHistoryVisibility(ref event) => event.serialize(serializer),
            RoomEvent::RoomJoinRules(ref event) => event.serialize(serializer),
            RoomEvent::RoomMember(ref event) => event.serialize(serializer),
            RoomEvent::RoomMessage(ref event) => event.serialize(serializer),
            RoomEvent::RoomMessageFeedback(ref event) => event.serialize(serializer),
            RoomEvent::RoomName(ref event) => event.serialize(serializer),
            RoomEvent::RoomPinnedEvents(ref event) => event.serialize(serializer),
            RoomEvent::RoomPowerLevels(ref event) => event.serialize(serializer),
            RoomEvent::RoomRedaction(ref event) => event.serialize(serializer),
            RoomEvent::RoomServerAcl(ref event) => event.serialize(serializer),
            RoomEvent::RoomThirdPartyInvite(ref event) => event.serialize(serializer),
            RoomEvent::RoomTombstone(ref event) => event.serialize(serializer),
            RoomEvent::RoomTopic(ref event) => event.serialize(serializer),
            RoomEvent::Sticker(ref event) => event.serialize(serializer),
            RoomEvent::CustomRoom(ref event) => event.serialize(serializer),
            RoomEvent::CustomState(ref event) => event.serialize(serializer),
        }
    }
}

impl Serialize for StateEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            StateEvent::RoomAliases(ref event) => event.serialize(serializer),
            StateEvent::RoomAvatar(ref event) => event.serialize(serializer),
            StateEvent::RoomCanonicalAlias(ref event) => event.serialize(serializer),
            StateEvent::RoomCreate(ref event) => event.serialize(serializer),
            StateEvent::RoomEncryption(ref event) => event.serialize(serializer),
            StateEvent::RoomGuestAccess(ref event) => event.serialize(serializer),
            StateEvent::RoomHistoryVisibility(ref event) => event.serialize(serializer),
            StateEvent::RoomJoinRules(ref event) => event.serialize(serializer),
            StateEvent::RoomMember(ref event) => event.serialize(serializer),
            StateEvent::RoomName(ref event) => event.serialize(serializer),
            StateEvent::RoomPinnedEvents(ref event) => event.serialize(serializer),
            StateEvent::RoomPowerLevels(ref event) => event.serialize(serializer),
            StateEvent::RoomServerAcl(ref event) => event.serialize(serializer),
            StateEvent::RoomThirdPartyInvite(ref event) => event.serialize(serializer),
            StateEvent::RoomTombstone(ref event) => event.serialize(serializer),
            StateEvent::RoomTopic(ref event) => event.serialize(serializer),
            StateEvent::CustomState(ref event) => event.serialize(serializer),
        }
    }
}

macro_rules! impl_from_t_for_event {
    ($ty:ty, $variant:ident) => {
        impl From<$ty> for Event {
            fn from(event: $ty) -> Self {
                Event::$variant(event)
            }
        }
    };
}

impl_from_t_for_event!(AnswerEvent, CallAnswer);
impl_from_t_for_event!(CandidatesEvent, CallCandidates);
impl_from_t_for_event!(HangupEvent, CallHangup);
impl_from_t_for_event!(InviteEvent, CallInvite);
impl_from_t_for_event!(DirectEvent, Direct);
impl_from_t_for_event!(DummyEvent, Dummy);
impl_from_t_for_event!(ForwardedRoomKeyEvent, ForwardedRoomKey);
impl_from_t_for_event!(FullyReadEvent, FullyRead);
impl_from_t_for_event!(AcceptEvent, KeyVerificationAccept);
impl_from_t_for_event!(CancelEvent, KeyVerificationCancel);
impl_from_t_for_event!(KeyEvent, KeyVerificationKey);
impl_from_t_for_event!(MacEvent, KeyVerificationMac);
impl_from_t_for_event!(RequestEvent, KeyVerificationRequest);
impl_from_t_for_event!(StartEvent, KeyVerificationStart);
impl_from_t_for_event!(IgnoredUserListEvent, IgnoredUserList);
impl_from_t_for_event!(PresenceEvent, Presence);
impl_from_t_for_event!(PushRulesEvent, PushRules);
impl_from_t_for_event!(ReceiptEvent, Receipt);
impl_from_t_for_event!(AliasesEvent, RoomAliases);
impl_from_t_for_event!(AvatarEvent, RoomAvatar);
impl_from_t_for_event!(CanonicalAliasEvent, RoomCanonicalAlias);
impl_from_t_for_event!(CreateEvent, RoomCreate);
impl_from_t_for_event!(EncryptedEvent, RoomEncrypted);
impl_from_t_for_event!(EncryptionEvent, RoomEncryption);
impl_from_t_for_event!(GuestAccessEvent, RoomGuestAccess);
impl_from_t_for_event!(HistoryVisibilityEvent, RoomHistoryVisibility);
impl_from_t_for_event!(JoinRulesEvent, RoomJoinRules);
impl_from_t_for_event!(MemberEvent, RoomMember);
impl_from_t_for_event!(MessageEvent, RoomMessage);
impl_from_t_for_event!(FeedbackEvent, RoomMessageFeedback);
impl_from_t_for_event!(NameEvent, RoomName);
impl_from_t_for_event!(PinnedEventsEvent, RoomPinnedEvents);
impl_from_t_for_event!(PowerLevelsEvent, RoomPowerLevels);
impl_from_t_for_event!(RedactionEvent, RoomRedaction);
impl_from_t_for_event!(ServerAclEvent, RoomServerAcl);
impl_from_t_for_event!(ThirdPartyInviteEvent, RoomThirdPartyInvite);
impl_from_t_for_event!(TombstoneEvent, RoomTombstone);
impl_from_t_for_event!(TopicEvent, RoomTopic);
impl_from_t_for_event!(RoomKeyEvent, RoomKey);
impl_from_t_for_event!(RoomKeyRequestEvent, RoomKeyRequest);
impl_from_t_for_event!(StickerEvent, Sticker);
impl_from_t_for_event!(TagEvent, Tag);
impl_from_t_for_event!(TypingEvent, Typing);
impl_from_t_for_event!(CustomEvent, Custom);
impl_from_t_for_event!(CustomRoomEvent, CustomRoom);
impl_from_t_for_event!(CustomStateEvent, CustomState);

macro_rules! impl_from_t_for_room_event {
    ($ty:ty, $variant:ident) => {
        impl From<$ty> for RoomEvent {
            fn from(event: $ty) -> Self {
                RoomEvent::$variant(event)
            }
        }
    };
}

impl_from_t_for_room_event!(AnswerEvent, CallAnswer);
impl_from_t_for_room_event!(CandidatesEvent, CallCandidates);
impl_from_t_for_room_event!(HangupEvent, CallHangup);
impl_from_t_for_room_event!(InviteEvent, CallInvite);
impl_from_t_for_room_event!(AliasesEvent, RoomAliases);
impl_from_t_for_room_event!(AvatarEvent, RoomAvatar);
impl_from_t_for_room_event!(CanonicalAliasEvent, RoomCanonicalAlias);
impl_from_t_for_room_event!(CreateEvent, RoomCreate);
impl_from_t_for_room_event!(EncryptedEvent, RoomEncrypted);
impl_from_t_for_room_event!(EncryptionEvent, RoomEncryption);
impl_from_t_for_room_event!(GuestAccessEvent, RoomGuestAccess);
impl_from_t_for_room_event!(HistoryVisibilityEvent, RoomHistoryVisibility);
impl_from_t_for_room_event!(JoinRulesEvent, RoomJoinRules);
impl_from_t_for_room_event!(MemberEvent, RoomMember);
impl_from_t_for_room_event!(MessageEvent, RoomMessage);
impl_from_t_for_room_event!(FeedbackEvent, RoomMessageFeedback);
impl_from_t_for_room_event!(NameEvent, RoomName);
impl_from_t_for_room_event!(PinnedEventsEvent, RoomPinnedEvents);
impl_from_t_for_room_event!(PowerLevelsEvent, RoomPowerLevels);
impl_from_t_for_room_event!(RedactionEvent, RoomRedaction);
impl_from_t_for_room_event!(ServerAclEvent, RoomServerAcl);
impl_from_t_for_room_event!(StickerEvent, Sticker);
impl_from_t_for_room_event!(ThirdPartyInviteEvent, RoomThirdPartyInvite);
impl_from_t_for_room_event!(TombstoneEvent, RoomTombstone);
impl_from_t_for_room_event!(TopicEvent, RoomTopic);
impl_from_t_for_room_event!(CustomRoomEvent, CustomRoom);
impl_from_t_for_room_event!(CustomStateEvent, CustomState);

macro_rules! impl_from_t_for_state_event {
    ($ty:ty, $variant:ident) => {
        impl From<$ty> for StateEvent {
            fn from(event: $ty) -> Self {
                StateEvent::$variant(event)
            }
        }
    };
}

impl_from_t_for_state_event!(AliasesEvent, RoomAliases);
impl_from_t_for_state_event!(AvatarEvent, RoomAvatar);
impl_from_t_for_state_event!(CanonicalAliasEvent, RoomCanonicalAlias);
impl_from_t_for_state_event!(CreateEvent, RoomCreate);
impl_from_t_for_state_event!(EncryptionEvent, RoomEncryption);
impl_from_t_for_state_event!(GuestAccessEvent, RoomGuestAccess);
impl_from_t_for_state_event!(HistoryVisibilityEvent, RoomHistoryVisibility);
impl_from_t_for_state_event!(JoinRulesEvent, RoomJoinRules);
impl_from_t_for_state_event!(MemberEvent, RoomMember);
impl_from_t_for_state_event!(NameEvent, RoomName);
impl_from_t_for_state_event!(PinnedEventsEvent, RoomPinnedEvents);
impl_from_t_for_state_event!(PowerLevelsEvent, RoomPowerLevels);
impl_from_t_for_state_event!(ServerAclEvent, RoomServerAcl);
impl_from_t_for_state_event!(ThirdPartyInviteEvent, RoomThirdPartyInvite);
impl_from_t_for_state_event!(TombstoneEvent, RoomTombstone);
impl_from_t_for_state_event!(TopicEvent, RoomTopic);
impl_from_t_for_state_event!(CustomStateEvent, CustomState);
