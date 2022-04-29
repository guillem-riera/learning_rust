#[cfg(test)]
use super::*; // Imports all items from the `version` module.

#[test]
fn version_test() {
    // Given expected version
    let expected = "2.0.0";

    // when I create a Version struct with this expected version
    let v = Version {
        version: expected.parse().unwrap(),
    };
    let actual = v.version;

    // Then
    assert_eq!(expected, actual)
}
