use super::{
    json::index::IndexJson,
    utils::{AsBoolean, Result, INDEX_JSON_URL},
};

#[test]
fn as_boolean() -> Result<()> {
    assert_eq!(0.as_boolean()?, false);
    assert_eq!(1.as_boolean()?, true);

    assert_eq!(2.as_boolean()?, true);
    assert_eq!(u8::MAX.as_boolean()?, true);

    Ok(())
}

#[tokio::test]
async fn get_index_json() -> Result<()> {
    for index_json_url in INDEX_JSON_URL {
        let index_json = get_response!(index.json, index_json_url);
        let index_json = serde_json::to_string_pretty(&index_json)?;

        println!("{index_json}");
    }

    Ok(())
}
