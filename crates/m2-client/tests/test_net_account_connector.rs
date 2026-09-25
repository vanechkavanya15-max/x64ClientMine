use m2_client::net_account_connector::NetAccountConnector;

#[test]
fn test_integration_net_account_connector() {
    let instance = NetAccountConnector::new();
    assert!(instance.is_ready());
}
