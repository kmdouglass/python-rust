#[cfg(test)]
use crate::meaning_of_life;

#[test]
fn test_meaning_of_life() {
    let result = meaning_of_life();
    assert_eq!(42, result);
}
