use tokio::test;

use crate::{Config::get_config, Db::init_csv};

#[test]
async fn test_init_csv() {
    let cfg = get_config();
    dbg!(cfg);
    init_csv().await.unwrap();
}
