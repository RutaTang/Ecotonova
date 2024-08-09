use crate::theory::pitch::{Pitch, PitchName};

#[derive(Debug, Clone, PartialEq)]
pub enum IntervalQuality {
    Perfect,
    Major,
    Minor,
    Augmented,
    Diminished,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntervalStep {
    Half,
    Whole,
}

impl TryFrom<f32> for IntervalStep {
    type Error = ();

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        match value {
            0.5 => Ok(IntervalStep::Half),
            1.0 => Ok(IntervalStep::Whole),
            _ => Err(()),
        }
    }
}

impl From<IntervalStep> for f32 {
    fn from(step: IntervalStep) -> f32 {
        match step {
            IntervalStep::Half => 0.5,
            IntervalStep::Whole => 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Interval {
    lower: Pitch,
    upper: Pitch,
}

impl Interval {
    pub fn new(p1: Pitch, p2: Pitch) -> Self {
        return if p1 < p2 {
            Self { lower: p1, upper: p2 }
        } else {
            Self { lower: p2, upper: p1 }
        };
    }

    /// Calculates the interval number.
    ///
    /// # Arguments
    ///
    /// * `ignore_octave` - A `bool` that indicates if the octave should be ignored.
    ///
    /// # Returns
    ///
    /// A `u8` representing the interval number.
    pub fn get_number(&self, ignore_octave: bool) -> u8 {
        let get_position = |pitch: &Pitch| -> u8 {
            match &pitch.name {
                PitchName::C => 0,
                PitchName::D => 1,
                PitchName::E => 2,
                PitchName::F => 3,
                PitchName::G => 4,
                PitchName::A => 5,
                PitchName::B => 6,
            }
        };
        let lower_position = get_position(&self.lower);
        let upper_position = get_position(&self.upper);
        let octave_diff = (self.upper.octave - self.lower.octave).abs() as u8 * 7;
        if ignore_octave {
            if upper_position < lower_position {
                upper_position + 7 - lower_position + 1
            } else {
                upper_position - lower_position + 1
            }
        } else {
            upper_position - lower_position + 1 + octave_diff
        }
    }

    /// Calculates the semitones between two pitches.
    ///
    /// # Arguments
    ///
    /// * `ignore_octave` - A `bool` that indicates if the octave should be ignored.
    ///
    /// # Returns
    ///
    /// A `u16` representing the n of semitones .
    pub fn get_number_of_semitones(&self, ignore_octave: bool) -> u16 {
        let semitones = ((f32::from(self.upper.clone()) - f32::from(self.lower.clone())) / f32::from(IntervalStep::Half)) as u16;
        if ignore_octave {
            let octave_diff = (self.upper.octave - self.lower.octave).abs() as f32;
            if semitones > 12 {
                semitones - (octave_diff * 12.0) as u16
            } else {
                semitones
            }
        } else {
            semitones
        }
    }

    /// Calculates the interval quality.
    ///
    /// # Returns
    ///
    /// A `IntervalQuality` representing the interval quality.
    pub fn get_quality(&self) -> Result<IntervalQuality, ()> {
        let number = self.get_number(true);
        let semitones = self.get_number_of_semitones(true);
        let quality = match number {
            1 => {
                return match semitones {
                    0 => Ok(IntervalQuality::Perfect),
                    1 => Ok(IntervalQuality::Augmented),
                    _ => Err(()),
                }
            }
            2 => {
                return match semitones {
                    0 => Ok(IntervalQuality::Diminished),
                    1 => Ok(IntervalQuality::Minor),
                    2 => Ok(IntervalQuality::Major),
                    3 => Ok(IntervalQuality::Augmented),
                    _ => Err(()),
                }
            }
            3 => {
                return match semitones {
                    2 => Ok(IntervalQuality::Diminished),
                    3 => Ok(IntervalQuality::Minor),
                    4 => Ok(IntervalQuality::Major),
                    5 => Ok(IntervalQuality::Augmented),
                    _ => Err(()),
                }
            }
            4 => {
                return match semitones {
                    4 => Ok(IntervalQuality::Diminished),
                    5 => Ok(IntervalQuality::Perfect),
                    6 => Ok(IntervalQuality::Augmented),
                    _ => Err(()),
                }
            }
            5 => {
                return match semitones {
                    6 => Ok(IntervalQuality::Diminished),
                    7 => Ok(IntervalQuality::Perfect),
                    8 => Ok(IntervalQuality::Augmented),
                    _ => Err(()),
                }
            }
            6 => {
                return match semitones {
                    7 => Ok(IntervalQuality::Diminished),
                    8 => Ok(IntervalQuality::Minor),
                    9 => Ok(IntervalQuality::Major),
                    10 => Ok(IntervalQuality::Augmented),
                    _ => Err(()),
                }
            }
            7 => {
                return match semitones {
                    9 => Ok(IntervalQuality::Diminished),
                    10 => Ok(IntervalQuality::Minor),
                    11 => Ok(IntervalQuality::Major),
                    12 => Ok(IntervalQuality::Augmented),
                    _ => Err(()),
                }
            }
            _ => Err(()),
        };
        return quality;
    }

    /// Calculates the specific interval.
    ///
    /// # Returns
    ///
    ///  A tuple containing the interval number, the interval quality and a boolean indicating if the interval is greater than an octave.
    pub fn get_specific_interval(&self) -> (u8, IntervalQuality, bool) {
        (
            self.get_number(true),
            self.get_quality().unwrap(),
            self.get_number_of_semitones(false) > 12,
        )
    }
}

#[cfg(test)]
mod get_number_tests {
    use super::*;

    #[test]
    fn test_in_the_same_octave() {
        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new_without_accidental(PitchName::E, 0);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_number(false), 3);

        let p1 = Pitch::new_without_accidental(PitchName::C, 2);
        let p2 = Pitch::new_without_accidental(PitchName::G, 2);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_number(false), 5);
    }

    #[test]
    fn test_in_different_octave() {
        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new_without_accidental(PitchName::E, 1);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_number(false), 10);

        let p1 = Pitch::new_without_accidental(PitchName::C, 1);
        let p2 = Pitch::new_without_accidental(PitchName::G, 3);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_number(false), 19);
    }

    #[test]
    fn test_ignore_octave() {
        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new_without_accidental(PitchName::E, 0);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_number(true), 3);

        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new_without_accidental(PitchName::E, 1);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_number(true), 3);

        let p1 = Pitch::new_without_accidental(PitchName::C, 1);
        let p2 = Pitch::new_without_accidental(PitchName::G, 3);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_number(true), 5);

        let p1 = Pitch::new_without_accidental(PitchName::C, 1);
        let p2 = Pitch::new_without_accidental(PitchName::B, 0);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_number(true), 2);

        let p1 = Pitch::new_without_accidental(PitchName::C, 1);
        let p2 = Pitch::new_without_accidental(PitchName::G, 0);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_number(true), 4);
    }
}

#[cfg(test)]
mod get_number_of_semitones_tests {
    use super::*;

    #[test]
    fn test_in_the_same_octave() {
        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new_without_accidental(PitchName::E, 0);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_number_of_semitones(false), 4);
    }

    #[test]
    fn test_in_different_octave() {
        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new_without_accidental(PitchName::E, 1);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_number_of_semitones(false), 16);

        let p1 = Pitch::new_without_accidental(PitchName::C, 1);
        let p2 = Pitch::new_without_accidental(PitchName::G, 3);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_number_of_semitones(false), 31);
    }

    #[test]
    fn test_ignore_octave() {
        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new_without_accidental(PitchName::E, 0);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_number_of_semitones(true), 4);

        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new_without_accidental(PitchName::E, 1);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_number_of_semitones(true), 4);

        let p1 = Pitch::new_without_accidental(PitchName::C, 1);
        let p2 = Pitch::new_without_accidental(PitchName::G, 3);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_number_of_semitones(true), 7);

        let p1 = Pitch::new_without_accidental(PitchName::C, 1);
        let p2 = Pitch::new_without_accidental(PitchName::B, 0);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_number_of_semitones(true), 1);

        let p1 = Pitch::new_without_accidental(PitchName::C, 1);
        let p2 = Pitch::new_without_accidental(PitchName::G, 0);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_number_of_semitones(true), 5);
    }
}

#[cfg(test)]
mod get_quality_tests {
    use crate::theory::pitch::Accidental;
    use super::*;

    #[test]
    fn test_perfect_intervals() {
        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new_without_accidental(PitchName::C, 0);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_quality(), Ok(IntervalQuality::Perfect));

        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new_without_accidental(PitchName::G, 0);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_quality(), Ok(IntervalQuality::Perfect));

        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new_without_accidental(PitchName::F, 0);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_quality(), Ok(IntervalQuality::Perfect));

        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new_without_accidental(PitchName::G, 1);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_quality(), Ok(IntervalQuality::Perfect));

        let p1 = Pitch::new_without_accidental(PitchName::C, 1);
        let p2 = Pitch::new_without_accidental(PitchName::G, 0);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_quality(), Ok(IntervalQuality::Perfect));
    }

    #[test]
    fn test_minor_intervals() {
        let p1 = Pitch::new_without_accidental(PitchName::E, 0);
        let p2 = Pitch::new_without_accidental(PitchName::F, 0);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_quality(), Ok(IntervalQuality::Minor));

        let p1 = Pitch::new_without_accidental(PitchName::B, 0);
        let p2 = Pitch::new_without_accidental(PitchName::C, 1);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_quality(), Ok(IntervalQuality::Minor));

        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new(PitchName::D, 0, Accidental::Flat);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_quality(), Ok(IntervalQuality::Minor));

        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new(PitchName::E, 0, Accidental::Flat);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_quality(), Ok(IntervalQuality::Minor));

        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new(PitchName::A, 0, Accidental::Flat);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_quality(), Ok(IntervalQuality::Minor));

        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new(PitchName::B, 0, Accidental::Flat);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_quality(), Ok(IntervalQuality::Minor));
    }

    #[test]
    fn test_major_intervals() {
        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new_without_accidental(PitchName::D, 0);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_quality(), Ok(IntervalQuality::Major));

        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new_without_accidental(PitchName::E, 0);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_quality(), Ok(IntervalQuality::Major));

        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new_without_accidental(PitchName::A, 0);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_quality(), Ok(IntervalQuality::Major));

        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new_without_accidental(PitchName::B, 0);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_quality(), Ok(IntervalQuality::Major));
    }

    #[test]
    fn test_diminished_intervals() {
        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new(PitchName::F, 0, Accidental::Flat);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_quality(), Ok(IntervalQuality::Diminished));

        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new(PitchName::G, 0, Accidental::Flat);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_quality(), Ok(IntervalQuality::Diminished));
    }

    #[test]
    fn test_augmented_intervals() {
        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new(PitchName::F, 0, Accidental::Sharp);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_quality(), Ok(IntervalQuality::Augmented));

        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new(PitchName::G, 0, Accidental::Sharp);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_quality(), Ok(IntervalQuality::Augmented));
    }
}

#[cfg(test)]
mod get_specific_interval_tests {
    use super::*;

    #[test]
    fn test_in_the_same_octave() {
        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new_without_accidental(PitchName::E, 0);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_specific_interval(), (3, IntervalQuality::Major, false));

        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new_without_accidental(PitchName::G, 0);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_specific_interval(), (5, IntervalQuality::Perfect, false));
    }

    #[test]
    fn test_in_different_octave() {
        let p1 = Pitch::new_without_accidental(PitchName::C, 0);
        let p2 = Pitch::new_without_accidental(PitchName::E, 1);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_specific_interval(), (3, IntervalQuality::Major, true));

        let p1 = Pitch::new_without_accidental(PitchName::C, 1);
        let p2 = Pitch::new_without_accidental(PitchName::G, 3);
        let interval = Interval::new(p1, p2);
        assert_eq!(interval.get_specific_interval(), (5, IntervalQuality::Perfect, true));
    }
}