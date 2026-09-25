use m2_client::app_procedure::AppProcedure;

#[test]
fn test_integration_app_procedure() {
    let instance = AppProcedure::new();
    assert!(instance.is_ready());
}
