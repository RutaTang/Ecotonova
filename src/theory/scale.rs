use crate::theory::interval::IntervalStep;


/// A scale is a collection of intervals that sum to 12.
///
/// `steps`: Vec<u8> - each item is the number of half steps
pub struct Scale {
    steps: Vec<u8>,
}

impl Scale {
    pub fn try_new<T>(steps: T) -> Result<Self, ()>
    where
        T: IntoIterator<Item=u8>,
    {
        let steps: Vec<u8> = steps.into_iter().collect();
        // Ensure all steps sum to 12
        let sum: f32 = steps.iter().fold(0.0, |acc, step| acc + f32::from(*step));
        if sum != 12.0 {
            return Err(());
        }
        Ok(Self { steps })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_new() {
        let scale = Scale::try_new(vec![
            2, 2, 1, 2, 2, 2, 1,
        ]);
        assert!(scale.is_ok());
    }
}