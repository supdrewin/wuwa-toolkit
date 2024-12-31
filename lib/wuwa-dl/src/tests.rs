use super::util::{AsBoolean, Result};

#[test]
fn as_boolean() -> Result<()> {
    assert_eq!(0.as_boolean()?, false);
    assert_eq!(1.as_boolean()?, true);

    assert_eq!(2.as_boolean()?, true);
    assert_eq!(u8::MAX.as_boolean()?, true);

    Ok(())
}
