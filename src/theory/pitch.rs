use std::fmt::{Display, Formatter};
use std::io::Error;
use std::ops::Sub;
use regex::Regex;
use crate::theory::interval::{Interval, IntervalStep};
use crate::utils::float_mod;

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum PitchName {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl Display for PitchName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PitchName::C => "C",
            PitchName::D => "D",
            PitchName::E => "E",
            PitchName::F => "F",
            PitchName::G => "G",
            PitchName::A => "A",
            PitchName::B => "B",
        })
    }
}

impl TryFrom<String> for PitchName {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "C" => Ok(PitchName::C),
            "D" => Ok(PitchName::D),
            "E" => Ok(PitchName::E),
            "F" => Ok(PitchName::F),
            "G" => Ok(PitchName::G),
            "A" => Ok(PitchName::A),
            "B" => Ok(PitchName::B),
            _ => Err(()),
        }
    }
}


#[derive(Clone, PartialEq, Debug, Eq)]
pub enum Accidental {
    Sharp,
    Flat,
    DoubleSharp,
    DoubleFlat,
    None,
}

impl Display for Accidental {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Accidental::Sharp => "#",
            Accidental::Flat => "b",
            Accidental::DoubleSharp => "##",
            Accidental::DoubleFlat => "bb",
            Accidental::None => "",
        })
    }
}

impl TryFrom<String> for Accidental {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "#" => Ok(Accidental::Sharp),
            "b" => Ok(Accidental::Flat),
            "##" => Ok(Accidental::DoubleSharp),
            "bb" => Ok(Accidental::DoubleFlat),
            "" => Ok(Accidental::None),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Pitch {
    pub name: PitchName,
    pub accidental: Accidental,
    pub octave: i8,
}

impl Display for Pitch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.name, self.accidental, self.octave)
    }
}

impl TryFrom<String> for Pitch {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let re = Regex::new(r"^([A-G])(#{1,2}|b{1,2})?([0-9]|[1-9]\d)$").unwrap();
        if let Some(captures) = re.captures(&value) {
            let name = captures.get(1).ok_or(())?.as_str();
            let accidental = captures.get(2).map_or("", |m| m.as_str());
            let octave = captures.get(3).ok_or(())?.as_str();
            Ok(Self {
                name: PitchName::try_from(name.to_string())?,
                accidental: Accidental::try_from(accidental.to_string())?,
                octave: octave.parse().map_err(|_| ())?,
            })
        } else {
            Err(())
        }
    }
}

impl Pitch {
    pub fn new(name: PitchName, octave: i8, accidental: Accidental) -> Self {
        Self {
            name,
            octave,
            accidental,
        }
    }
    pub fn new_without_accidental(name: PitchName, octave: i8) -> Self {
        Self {
            name,
            octave,
            accidental: Accidental::None,
        }
    }
    pub fn to_hertz(&self) -> f32 {
        let standard_pitch = Pitch::new_without_accidental(PitchName::A, 4);
        let number_of_semitones = (f32::from(self.clone()) - f32::from(standard_pitch)) / f32::from(IntervalStep::Half);
        440.0 * 2.0_f32.powf(number_of_semitones / 12.0)
    }
    pub fn distance(&self, other: &Self) -> f32 {
        let dist = f32::from(other.clone()) - f32::from(self.clone());
        dist.abs()
    }
    pub fn get_the_nearest_pitch(&self, others: Vec<Pitch>) -> Self {
        let mut min_distance = f32::MAX;
        let mut nearest_pitch = self.clone();
        for pitch in others {
            let distance = self.distance(&pitch);
            if distance < min_distance {
                min_distance = distance;
                nearest_pitch = pitch;
            }
        }
        nearest_pitch
    }
}


impl PartialEq<Self> for Pitch {
    fn eq(&self, other: &Self) -> bool {
        let left = f32::from(self.clone());
        let right = f32::from(other.clone());
        left == right
    }
}

impl Eq for Pitch {}

impl PartialOrd<Self> for Pitch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let left = f32::from(self.clone());
        let right = f32::from(other.clone());
        left.partial_cmp(&right)
    }
}

impl Ord for Pitch {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let left = f32::from(self.clone());
        let right = f32::from(other.clone());
        left.partial_cmp(&right).unwrap()
    }
}

impl TryFrom<f32> for Pitch {
    type Error = ();

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        let (pitch_name, accidental) = match float_mod(value, 6.0) {
            0.0 => (PitchName::C, Accidental::None),
            0.5 => (PitchName::C, Accidental::Sharp),
            1.0 => (PitchName::D, Accidental::None),
            1.5 => (PitchName::D, Accidental::Sharp),
            2.0 => (PitchName::E, Accidental::None),
            2.5 => (PitchName::F, Accidental::None),
            3.0 => (PitchName::F, Accidental::Sharp),
            3.5 => (PitchName::G, Accidental::None),
            4.0 => (PitchName::G, Accidental::Sharp),
            4.5 => (PitchName::A, Accidental::None),
            5.0 => (PitchName::A, Accidental::Sharp),
            5.5 => (PitchName::B, Accidental::None),
            _ => return Err(()),
        };
        let octave = (value / 6.0).floor() as i8;
        Ok(Self::new(pitch_name, octave, accidental))
    }
}

impl From<Pitch> for f32 {
    fn from(value: Pitch) -> Self {
        let pitch_value = match value.name {
            PitchName::C => 0.0,
            PitchName::D => 1.0,
            PitchName::E => 2.0,
            PitchName::F => 2.5,
            PitchName::G => 3.5,
            PitchName::A => 4.5,
            PitchName::B => 5.5,
        };
        let accidental_value = match value.accidental {
            Accidental::Sharp => 0.5,
            Accidental::Flat => -0.5,
            Accidental::DoubleSharp => 1.0,
            Accidental::DoubleFlat => -1.0,
            Accidental::None => 0.0,
        };
        (pitch_value + accidental_value) + (value.octave as f32 * 6.0)
    }
}


#[cfg(test)]
mod pitch_try_from_f32_tests {
    use super::*;

    #[test]
    fn test_zero_octave() {
        assert_eq!(Pitch::try_from(0.0).unwrap(), Pitch::new_without_accidental(PitchName::C, 0));
        assert_eq!(Pitch::try_from(1.0).unwrap(), Pitch::new_without_accidental(PitchName::D, 0));
        assert_eq!(Pitch::try_from(2.0).unwrap(), Pitch::new_without_accidental(PitchName::E, 0));
        assert_eq!(Pitch::try_from(2.5).unwrap(), Pitch::new_without_accidental(PitchName::F, 0));
        assert_eq!(Pitch::try_from(3.5).unwrap(), Pitch::new_without_accidental(PitchName::G, 0));
        assert_eq!(Pitch::try_from(4.5).unwrap(), Pitch::new_without_accidental(PitchName::A, 0));
        assert_eq!(Pitch::try_from(5.5).unwrap(), Pitch::new_without_accidental(PitchName::B, 0));
    }

    #[test]
    fn test_other_octaves() {
        assert_eq!(Pitch::try_from(6.0).unwrap(), Pitch::new_without_accidental(PitchName::C, 1));
        assert_eq!(Pitch::try_from(7.0).unwrap(), Pitch::new_without_accidental(PitchName::D, 1));
        assert_eq!(Pitch::try_from(8.0).unwrap(), Pitch::new_without_accidental(PitchName::E, 1));
        assert_eq!(Pitch::try_from(8.5).unwrap(), Pitch::new_without_accidental(PitchName::F, 1));
        assert_eq!(Pitch::try_from(9.5).unwrap(), Pitch::new_without_accidental(PitchName::G, 1));
        assert_eq!(Pitch::try_from(10.5).unwrap(), Pitch::new_without_accidental(PitchName::A, 1));
        assert_eq!(Pitch::try_from(11.5).unwrap(), Pitch::new_without_accidental(PitchName::B, 1));
    }
}

#[cfg(test)]
mod f32_from_pitch_tests {
    use super::*;

    #[test]
    fn test_zero_octave() {
        assert_eq!(f32::from(Pitch::new_without_accidental(PitchName::C, 0)), 0.0);
        assert_eq!(f32::from(Pitch::new_without_accidental(PitchName::D, 0)), 1.0);
        assert_eq!(f32::from(Pitch::new_without_accidental(PitchName::E, 0)), 2.0);
        assert_eq!(f32::from(Pitch::new_without_accidental(PitchName::F, 0)), 2.5);
        assert_eq!(f32::from(Pitch::new_without_accidental(PitchName::G, 0)), 3.5);
        assert_eq!(f32::from(Pitch::new_without_accidental(PitchName::A, 0)), 4.5);
        assert_eq!(f32::from(Pitch::new_without_accidental(PitchName::B, 0)), 5.5);
    }

    #[test]
    fn test_other_octaves() {
        assert_eq!(f32::from(Pitch::new_without_accidental(PitchName::C, 1)), 6.0);
        assert_eq!(f32::from(Pitch::new_without_accidental(PitchName::D, 1)), 7.0);
        assert_eq!(f32::from(Pitch::new_without_accidental(PitchName::E, 1)), 8.0);
        assert_eq!(f32::from(Pitch::new_without_accidental(PitchName::F, 1)), 8.5);
        assert_eq!(f32::from(Pitch::new_without_accidental(PitchName::G, 1)), 9.5);
        assert_eq!(f32::from(Pitch::new_without_accidental(PitchName::A, 1)), 10.5);
        assert_eq!(f32::from(Pitch::new_without_accidental(PitchName::B, 1)), 11.5);
    }

    #[test]
    fn test_with_accidentals() {
        assert_eq!(f32::from(Pitch::new(PitchName::C, 0, Accidental::Sharp)), 0.5);
        assert_eq!(f32::from(Pitch::new(PitchName::C, 0, Accidental::Flat)), -0.5);
        assert_eq!(f32::from(Pitch::new(PitchName::C, 0, Accidental::DoubleSharp)), 1.0);
        assert_eq!(f32::from(Pitch::new(PitchName::C, 0, Accidental::DoubleFlat)), -1.0);
    }
}

#[cfg(test)]
mod pitch_eq_tests {
    use super::*;

    #[test]
    fn test_eq_the_same() {
        let pitch1 = Pitch::new_without_accidental(PitchName::C, 0);
        let pitch2 = Pitch::new_without_accidental(PitchName::C, 0);
        assert_eq!(pitch1, pitch2);
        let pitch1 = Pitch::new_without_accidental(PitchName::C, 1);
        let pitch2 = Pitch::new_without_accidental(PitchName::C, 1);
        assert_eq!(pitch1, pitch2);
    }

    #[test]
    fn test_eq_different_accidental() {
        let pitch1 = Pitch::new(PitchName::C, 0, Accidental::Sharp);
        let pitch2 = Pitch::new(PitchName::D, 0, Accidental::Flat);
        assert_eq!(pitch1, pitch2);
        let pitch1 = Pitch::new(PitchName::C, 1, Accidental::Sharp);
        let pitch2 = Pitch::new(PitchName::D, 1, Accidental::Flat);
        assert_eq!(pitch1, pitch2);
    }

    #[test]
    fn test_ne() {
        let pitch1 = Pitch::new_without_accidental(PitchName::C, 0);
        let pitch2 = Pitch::new_without_accidental(PitchName::D, 0);
        assert_ne!(pitch1, pitch2);
        let pitch1 = Pitch::new_without_accidental(PitchName::C, 0);
        let pitch2 = Pitch::new_without_accidental(PitchName::C, 1);
        assert_ne!(pitch1, pitch2);
    }
}

#[cfg(test)]
mod to_hertz_tests {
    use super::*;

    #[test]
    fn test_a4() {
        let pitch = Pitch::new_without_accidental(PitchName::A, 4);
        assert_eq!(pitch.to_hertz(), 440.0);
    }

    #[test]
    fn test_a3() {
        let pitch = Pitch::new_without_accidental(PitchName::A, 3);
        assert_eq!(pitch.to_hertz(), 220.0);
    }

    #[test]
    fn test_c4() {
        let pitch = Pitch::new_without_accidental(PitchName::C, 4);
        assert!(pitch.to_hertz() - 261.62 < 0.01);
    }

    #[test]
    fn test_a4_accidentals() {
        let pitch = Pitch::new(PitchName::A, 4, Accidental::Sharp);
        assert!(pitch.to_hertz() - 466.16 < 0.01);
        let pitch = Pitch::new(PitchName::A, 4, Accidental::Flat);
        assert!(pitch.to_hertz() - 415.30 < 0.01);
        let pitch = Pitch::new(PitchName::A, 4, Accidental::DoubleSharp);
        assert!(pitch.to_hertz() - 493.88 < 0.01);
        let pitch = Pitch::new(PitchName::A, 4, Accidental::DoubleFlat);
        assert!(pitch.to_hertz() - 391.99 < 0.01);
    }

    #[test]
    fn test_a_minus1() {
        let pitch = Pitch::new_without_accidental(PitchName::A, -1);
        assert!(pitch.to_hertz() - 13.75 < 0.01);
    }
}

#[cfg(test)]
mod cmp_tests {
    use super::*;

    #[test]
    fn test_cmp() {
        let pitch1 = Pitch::new_without_accidental(PitchName::C, 0);
        let pitch2 = Pitch::new_without_accidental(PitchName::C, 0);
        assert!(pitch1 <= pitch2);
        assert!(pitch2 <= pitch1);

        let pitch1 = Pitch::new_without_accidental(PitchName::C, 0);
        let pitch2 = Pitch::new_without_accidental(PitchName::D, 0);
        assert!(pitch1 < pitch2);
        assert!(pitch2 > pitch1);
    }
}

#[cfg(test)]
mod display_tests {
    use super::*;

    #[test]
    fn test_display() {
        let pitch = Pitch::new_without_accidental(PitchName::C, 0);
        assert_eq!(format!("{}", pitch), "C0");
        let pitch = Pitch::new_without_accidental(PitchName::C, 1);
        assert_eq!(format!("{}", pitch), "C1");
        let pitch = Pitch::new(PitchName::C, 0, Accidental::Sharp);
        assert_eq!(format!("{}", pitch), "C#0");
        let pitch = Pitch::new(PitchName::C, 0, Accidental::Flat);
        assert_eq!(format!("{}", pitch), "Cb0");
        let pitch = Pitch::new(PitchName::C, 0, Accidental::DoubleSharp);
        assert_eq!(format!("{}", pitch), "C##0");
        let pitch = Pitch::new(PitchName::C, 0, Accidental::DoubleFlat);
        assert_eq!(format!("{}", pitch), "Cbb0");
    }
}

#[cfg(test)]
mod pitch_from_string_tests {
    use super::*;

    #[test]
    fn test_c0() {
        let pitch = Pitch::try_from("C0".to_string()).unwrap();
        assert_eq!(pitch, Pitch::new_without_accidental(PitchName::C, 0));
    }

    #[test]
    fn test_c1() {
        let pitch = Pitch::try_from("C1".to_string()).unwrap();
        assert_eq!(pitch, Pitch::new_without_accidental(PitchName::C, 1));
    }

    #[test]
    fn test_c0_sharp() {
        let pitch = Pitch::try_from("C#0".to_string()).unwrap();
        assert_eq!(pitch, Pitch::new(PitchName::C, 0, Accidental::Sharp));
    }

    #[test]
    fn test_c0_flat() {
        let pitch = Pitch::try_from("Cb0".to_string()).unwrap();
        assert_eq!(pitch, Pitch::new(PitchName::C, 0, Accidental::Flat));
    }

    #[test]
    fn test_c0_double_sharp() {
        let pitch = Pitch::try_from("C##0".to_string()).unwrap();
        assert_eq!(pitch, Pitch::new(PitchName::C, 0, Accidental::DoubleSharp));
    }

    #[test]
    fn test_c0_double_flat() {
        let pitch = Pitch::try_from("Cbb0".to_string()).unwrap();
        assert_eq!(pitch, Pitch::new(PitchName::C, 0, Accidental::DoubleFlat));
    }
}

#[cfg(test)]
mod get_the_nearest_pitch_tests {
    use crate::theory::pitch::Accidental::{None, Sharp};
    use super::*;

    #[test]
    fn test_in_the_same_octave() {
        let pitch = Pitch::new_without_accidental(PitchName::C, 0);
        let pitches = vec![
            Pitch::new(PitchName::C, 0, Sharp),
            Pitch::new_without_accidental(PitchName::D, 0),
            Pitch::new_without_accidental(PitchName::E, 0),
            Pitch::new_without_accidental(PitchName::F, 0),
            Pitch::new_without_accidental(PitchName::G, 0),
            Pitch::new_without_accidental(PitchName::A, 0),
            Pitch::new_without_accidental(PitchName::B, 0),
        ];
        assert_eq!(pitch.get_the_nearest_pitch(pitches), Pitch::new(PitchName::C, 0, Sharp));
    }

    #[test]
    fn test_in_different_octave() {
        let pitch = Pitch::new_without_accidental(PitchName::C, 1);
        let pitches = vec![
            Pitch::new_without_accidental(PitchName::C, 0),
            Pitch::new_without_accidental(PitchName::D, 0),
            Pitch::new_without_accidental(PitchName::E, 0),
            Pitch::new_without_accidental(PitchName::F, 0),
            Pitch::new_without_accidental(PitchName::G, 0),
            Pitch::new_without_accidental(PitchName::A, 0),
            Pitch::new_without_accidental(PitchName::B, 0),
        ];
        assert_eq!(pitch.get_the_nearest_pitch(pitches), Pitch::new_without_accidental(PitchName::B, 0));
    }

    #[test]
    fn test_the_nearest_is_self() {
        let pitch = Pitch::new_without_accidental(PitchName::C, 0);
        let pitches = vec![
            Pitch::new_without_accidental(PitchName::C, 0),
            Pitch::new_without_accidental(PitchName::D, 0),
            Pitch::new_without_accidental(PitchName::E, 0),
            Pitch::new_without_accidental(PitchName::F, 0),
            Pitch::new_without_accidental(PitchName::G, 0),
            Pitch::new_without_accidental(PitchName::A, 0),
            Pitch::new_without_accidental(PitchName::B, 0),
        ];
        assert_eq!(pitch.get_the_nearest_pitch(pitches), Pitch::new_without_accidental(PitchName::C, 0));
    }

    #[test]
    fn test_empty_pitches() {
        let pitch = Pitch::new_without_accidental(PitchName::C, 0);
        let pitches = vec![];
        assert_eq!(pitch.get_the_nearest_pitch(pitches), Pitch::new_without_accidental(PitchName::C, 0));
    }
}