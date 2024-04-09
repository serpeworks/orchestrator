mod communication_fixture;

#[test]
fn communication_system_register_session() {
    // Given
    let mut fixture = communication_fixture::CommunicationFixture::new();

    // When
    fixture.run_schedule();

    // Then
}
