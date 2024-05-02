use crate::{
    core::communication::SessionConnection,
    dialects::{serpe_dialect, SerpeDialect},
};

mod communication_fixture;

#[test]
fn receive_register_creates_entity() {
    // Given
    let mut fixture = communication_fixture::CommunicationFixture::new();
    let agent_id = 1;

    // When
    let channels = fixture.connect_session(agent_id);
    fixture.run_schedule();

    // Then
    let number_of_entities = fixture.world.entities().len();
    assert_eq!(number_of_entities, 1);
    let mut query = fixture.world.query::<&SessionConnection>();
    let _ = query.single(&fixture.world);
    drop(channels);
}

#[test]
fn registering_session_first_response_is_acknowledge() {
    // Given
    let mut fixture = communication_fixture::CommunicationFixture::new();
    let agent_id = 250;

    // When
    let (_sender, mut receiver) = fixture.connect_session(agent_id);
    fixture.run_schedule();

    // Then: can read attributed system_id
    let msg = receiver.try_recv().expect("Expected to receive a message");
    if let SerpeDialect::RegisterAck(msg) = msg {
        assert_ne!(msg.system_id, 0);
    } else {
        panic!("Expected RegisterAck message");
    }
}

#[test]
fn unregister_message_empties_world() {
    // Given
    let mut fixture = communication_fixture::CommunicationFixture::new();
    let agent_id = 1;
    let (sender, mut receiver) = fixture.connect_session(agent_id);
    fixture.run_schedule();

    // When
    let message = serpe_dialect::messages::Unregister {};
    sender
        .try_send(SerpeDialect::Unregister(message))
        .expect("Message send failed");
    fixture.run_schedule();
    let _ = receiver.try_recv().expect("Expected message"); // register ack

    // Then
    let number_of_entities = fixture.world.entities().len();
    assert_eq!(number_of_entities, 0);

    let msg = receiver.try_recv().expect("Expected message");
    if let SerpeDialect::UnregisterAck(_) = msg {
    } else {
        panic!("Expected UnregisterAck message");
    }
}

#[test]
fn session_loses_connection_updates_connection_status() {}
