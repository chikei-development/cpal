use std::convert::TryInto;
use std::time::Duration;

extern crate ndk;

use ndk::audio::AudioInputPreset;

use crate::{
    BackendSpecificError, BuildStreamError, PauseStreamError, PlayStreamError, StreamError,
    StreamInstant,
};

pub fn to_stream_instant(duration: Duration) -> StreamInstant {
    StreamInstant::new(
        duration.as_secs().try_into().unwrap(),
        duration.subsec_nanos(),
    )
}

pub fn stream_instant(stream: &ndk::audio::AudioStream) -> StreamInstant {
    let ts = stream
        .timestamp(ndk::audio::Clockid::Monotonic)
        .unwrap_or(ndk::audio::Timestamp {
            frame_position: 0,
            time_nanoseconds: 0,
        });
    to_stream_instant(Duration::from_nanos(ts.time_nanoseconds as u64))
}

impl From<ndk::audio::AudioError> for StreamError {
    fn from(error: ndk::audio::AudioError) -> Self {
        use self::ndk::audio::AudioError::*;
        match error {
            Disconnected | Unavailable => Self::DeviceNotAvailable,
            e => (BackendSpecificError {
                description: e.to_string(),
            })
            .into(),
        }
    }
}

impl From<ndk::audio::AudioError> for PlayStreamError {
    fn from(error: ndk::audio::AudioError) -> Self {
        use self::ndk::audio::AudioError::*;
        match error {
            Disconnected | Unavailable => Self::DeviceNotAvailable,
            e => (BackendSpecificError {
                description: e.to_string(),
            })
            .into(),
        }
    }
}

impl From<ndk::audio::AudioError> for PauseStreamError {
    fn from(error: ndk::audio::AudioError) -> Self {
        use self::ndk::audio::AudioError::*;
        match error {
            Disconnected | Unavailable => Self::DeviceNotAvailable,
            e => (BackendSpecificError {
                description: e.to_string(),
            })
            .into(),
        }
    }
}

impl From<ndk::audio::AudioError> for BuildStreamError {
    fn from(error: ndk::audio::AudioError) -> Self {
        use self::ndk::audio::AudioError::*;
        match error {
            Disconnected | Unavailable => Self::DeviceNotAvailable,
            NoFreeHandles => Self::StreamIdOverflow,
            InvalidFormat | InvalidRate => Self::StreamConfigNotSupported,
            IllegalArgument => Self::InvalidArgument,
            e => (BackendSpecificError {
                description: e.to_string(),
            })
            .into(),
        }
    }
}

impl From<crate::Usage> for ndk::audio::AudioUsage {
    fn from(value: crate::Usage) -> Self {
        match value {
            crate::Usage::Normal => Self::Media,
            crate::Usage::PhoneCall => Self::VoiceCommunication,
        }
    }
}

impl From<crate::Usage> for ndk::audio::AudioInputPreset {
    fn from(value: crate::Usage) -> Self {
        match value {
            crate::Usage::Normal => AudioInputPreset::Generic,
            crate::Usage::PhoneCall => AudioInputPreset::VoiceCommunication,
        }
    }
}
