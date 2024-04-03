// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /home/eitan/.local/share/flatpak/exports/share/gir-1.0
// from /home/eitan/Projects/spiel/libspeechprovider/build/libspeechprovider
// from /usr/local/share/gir-1.0
// from /usr/share/gir-1.0
// from /var/lib/flatpak/exports/share/gir-1.0
// DO NOT EDIT

use glib::{translate::*};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "SpeechProviderEventType")]
pub enum EventType {
    #[doc(alias = "SPEECH_PROVIDER_EVENT_TYPE_NONE")]
    None,
    #[doc(alias = "SPEECH_PROVIDER_EVENT_TYPE_WORD")]
    Word,
    #[doc(alias = "SPEECH_PROVIDER_EVENT_TYPE_SENTENCE")]
    Sentence,
    #[doc(alias = "SPEECH_PROVIDER_EVENT_TYPE_RANGE")]
    Range,
    #[doc(alias = "SPEECH_PROVIDER_EVENT_TYPE_MARK")]
    Mark,
#[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for EventType {
    type GlibType = ffi::SpeechProviderEventType;

    #[inline]
fn into_glib(self) -> ffi::SpeechProviderEventType {
match self {
            Self::None => ffi::SPEECH_PROVIDER_EVENT_TYPE_NONE,
            Self::Word => ffi::SPEECH_PROVIDER_EVENT_TYPE_WORD,
            Self::Sentence => ffi::SPEECH_PROVIDER_EVENT_TYPE_SENTENCE,
            Self::Range => ffi::SPEECH_PROVIDER_EVENT_TYPE_RANGE,
            Self::Mark => ffi::SPEECH_PROVIDER_EVENT_TYPE_MARK,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::SpeechProviderEventType> for EventType {
    #[inline]
unsafe fn from_glib(value: ffi::SpeechProviderEventType) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::SPEECH_PROVIDER_EVENT_TYPE_NONE => Self::None,
            ffi::SPEECH_PROVIDER_EVENT_TYPE_WORD => Self::Word,
            ffi::SPEECH_PROVIDER_EVENT_TYPE_SENTENCE => Self::Sentence,
            ffi::SPEECH_PROVIDER_EVENT_TYPE_RANGE => Self::Range,
            ffi::SPEECH_PROVIDER_EVENT_TYPE_MARK => Self::Mark,
            value => Self::__Unknown(value),
}
}
}

