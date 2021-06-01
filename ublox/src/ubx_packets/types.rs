use super::packets::*;
use crate::error::DateTimeError;
use chrono::prelude::*;
use core::convert::TryFrom;

/// Represents a world position, can be constructed from NavPosLlh and NavPosVelTime packets.
#[derive(Debug, Clone, Copy)]
pub struct Position {
    /// Logitude in degrees
    #[cfg(not(feature = "fixed-point"))]
    pub lon: f64,

    /// Logitude in degrees
    #[cfg(feature = "fixed-point")]
    pub lon: i32,

    /// Latitude in degrees
    #[cfg(not(feature = "fixed-point"))]
    pub lat: f64,

    /// Latitude in degrees
    #[cfg(feature = "fixed-point")]
    pub lat: i32,

    /// Altitude in meters
    #[cfg(not(feature = "fixed-point"))]
    pub alt: f64,

    /// Altitude in meters
    #[cfg(feature = "fixed-point")]
    pub alt: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    /// m/s over the ground
    #[cfg(not(feature = "fixed-point"))]
    pub speed: f64,

    /// m/s over the ground
    #[cfg(feature = "fixed-point")]
    pub speed: u32,

    /// Heading in degrees
    #[cfg(not(feature = "fixed-point"))]
    pub heading: f64, // degrees

    /// Heading in degrees
    #[cfg(feature = "fixed-point")]
    pub heading: i32, // degrees
}

impl<'a> From<&NavPosLlhRef<'a>> for Position {
    fn from(packet: &NavPosLlhRef<'a>) -> Self {
        Position {
            lon: packet.lon_degrees(),
            lat: packet.lat_degrees(),
            alt: packet.height_msl(),
        }
    }
}

impl<'a> From<&NavVelNedRef<'a>> for Velocity {
    fn from(packet: &NavVelNedRef<'a>) -> Self {
        Velocity {
            speed: packet.ground_speed(),
            heading: packet.heading_degrees(),
        }
    }
}

impl<'a> From<&NavPosVelTimeRef<'a>> for Position {
    fn from(packet: &NavPosVelTimeRef<'a>) -> Self {
        Position {
            lon: packet.lon_degrees(),
            lat: packet.lat_degrees(),
            alt: packet.height_msl(),
        }
    }
}

impl<'a> From<&NavPosVelTimeRef<'a>> for Velocity {
    fn from(packet: &NavPosVelTimeRef<'a>) -> Self {
        Velocity {
            speed: packet.ground_speed(),
            heading: packet.heading_degrees(),
        }
    }
}

impl<'a> TryFrom<&NavPosVelTimeRef<'a>> for DateTime<Utc> {
    type Error = DateTimeError;
    fn try_from(sol: &NavPosVelTimeRef<'a>) -> Result<Self, Self::Error> {
        let date = NaiveDate::from_ymd_opt(
            i32::from(sol.year()),
            u32::from(sol.month()),
            u32::from(sol.day()),
        )
        .ok_or(DateTimeError::InvalidDate)?;
        let time = NaiveTime::from_hms_opt(
            u32::from(sol.hour()),
            u32::from(sol.min()),
            u32::from(sol.sec()),
        )
        .ok_or(DateTimeError::InvalidTime)?;
        const NANOS_LIM: u32 = 1_000_000_000;
        if (sol.nanosecond().wrapping_abs() as u32) >= NANOS_LIM {
            return Err(DateTimeError::InvalidNanoseconds);
        }

        let dt = NaiveDateTime::new(date, time)
            + chrono::Duration::nanoseconds(i64::from(sol.nanosecond()));

        Ok(DateTime::from_utc(dt, Utc))
    }
}
