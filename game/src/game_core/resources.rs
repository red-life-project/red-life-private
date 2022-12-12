//! Contains the resources in the game(oxygen, energy and life and their associated functions)
//! Author: ["Benedikt Brandmaier", "Maximilian Floto", "Marion Hinkel", "Sander Stella"]
use crate::game_core::infoscreen::DeathReason;
use serde::{Deserialize, Serialize};
use std::cmp::PartialOrd;
use std::ops;

/// This struct holds data for resources
/// This is used to describe the current state and change rate of the player's resources
/// Use type `i16` for the resource change rate
/// Use type `u16` for the resource amount
#[derive(Copy, Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resources<T: PartialOrd> {
    pub(crate) oxygen: T,
    pub(crate) energy: T,
    pub(crate) life: T,
}

/// Define how struct will be created from an iterator
impl FromIterator<u16> for Resources<u16> {
    fn from_iter<I: IntoIterator<Item = u16>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        Self {
            oxygen: iter.next().unwrap(),
            energy: iter.next().unwrap(),
            life: iter.next().unwrap(),
        }
    }
}

/// Define how resources can be compared
impl<T: PartialOrd> IntoIterator for Resources<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [self.oxygen, self.energy, self.life].into_iter()
    }
}

/// Implement the `Add` trait for `Resources`
/// Use '+' to add two `Resources` of the same type
impl<T: ops::Add<Output = T> + PartialOrd> ops::Add<Resources<T>> for Resources<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            oxygen: self.oxygen + rhs.oxygen,
            energy: self.energy + rhs.energy,
            life: self.life + rhs.life,
        }
    }
}

/// Implement the `Sub` trait for `Resources`
/// Use '-' to substract two `Resources` of the same type from each other
impl<T: ops::Sub<Output = T> + PartialOrd> ops::Sub<Resources<T>> for Resources<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            oxygen: self.oxygen - rhs.oxygen,
            energy: self.energy - rhs.energy,
            life: self.life - rhs.life,
        }
    }
}

/// Implement function to check on death reason for `Resources` of type `u16`
impl Resources<u16> {
    /// This function returns the value that reached zero first
    /// If no value reached zero, it returns None
    pub fn get_death_reason(self) -> Option<DeathReason> {
        if self.oxygen == 0 && self.energy == 0 {
            Some(DeathReason::Both)
        } else if self.oxygen == 0 {
            Some(DeathReason::Oxygen)
        } else if self.energy == 0 {
            Some(DeathReason::Energy)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn addition() {
        let a = Resources {
            oxygen: 1,
            energy: 2,
            life: 3,
        };
        let b = Resources {
            oxygen: 4,
            energy: 5,
            life: 6,
        };
        let add_result = a + b;
        let add_control = Resources {
            oxygen: 5,
            energy: 7,
            life: 9,
        };
        assert_eq!(add_result, add_control);
    }

    #[test]
    fn subtraction() {
        let a = Resources {
            oxygen: 1,
            energy: 2,
            life: 3,
        };
        let b = Resources {
            oxygen: 4,
            energy: 5,
            life: 6,
        };
        let sub_result = a - b;
        let sub_control = Resources {
            oxygen: -3,
            energy: -3,
            life: -3,
        };
        assert_eq!(sub_result, sub_control);
    }

    #[test]
    fn into_it() {
        let a = Resources {
            oxygen: 3,
            energy: 2,
            life: 1,
        };
        let mut ait = a.into_iter();
        assert_eq!(ait.next().unwrap(), 3);
        assert_eq!(ait.next().unwrap(), 2);
        assert_eq!(ait.next().unwrap(), 1);
        assert_eq!(Some(ait.next()), Some(None));
    }
}
