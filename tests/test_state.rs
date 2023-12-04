//! Integration tests for application state handling.
mod common;

use babyrs::models::NewBabyEvent;
use babyrs::terminal;
use babyrs::{create_event, establish_connection, write_event};
use diesel::prelude::*;
use ratatui::widgets::ListState;

#[test]
fn test_load_events() {
    std::env::set_var("DATABASE_URL", ":memory:");

    let connection: &mut SqliteConnection = &mut establish_connection();

    common::run_migrations(connection).expect("Error running migrations");

    let new_event: NewBabyEvent = create_event(
        Some(true),
        Some(true),
        Some(5),
        Some(10),
        Some(15),
        Some(20),
        Some(25),
    );

    write_event(connection, new_event);

    let mut state = terminal::state::AppState::initialized();

    state.load_events(Some(connection));

    let results = state.get_events().expect("Error loading events");
    assert_eq!(results.len(), 1);
}

#[test]
fn test_get_events() {
    std::env::set_var("DATABASE_URL", ":memory:");

    let connection: &mut SqliteConnection = &mut establish_connection();

    common::run_migrations(connection).expect("Error running migrations");

    let new_event: NewBabyEvent = create_event(
        Some(true),
        Some(true),
        Some(5),
        Some(10),
        Some(15),
        Some(20),
        Some(25),
    );

    write_event(connection, new_event);

    let mut state = terminal::state::AppState::initialized();

    state.load_events(Some(connection));

    let results = state.get_events().expect("Error loading events");
    assert_eq!(results.len(), 1);

    let event = &results[0];
    assert_eq!(event.urine, true);
    assert_eq!(event.stool, true);
    assert_eq!(event.skin2skin, 5);
    assert_eq!(event.breastfeed, 10);
    assert_eq!(event.breastmilk, 15);
    assert_eq!(event.formula, 20);
    assert_eq!(event.pump, 25);
}

#[test]
fn test_get_selection() {
    std::env::set_var("DATABASE_URL", ":memory:");
    let mut state = terminal::state::AppState::initialized();
    let mut test_state = ListState::default();
    let connection: &mut SqliteConnection = &mut establish_connection();

    common::run_migrations(connection).expect("Error running migrations");

    let new_event: NewBabyEvent = create_event(
        Some(true),
        Some(true),
        Some(5),
        Some(10),
        Some(15),
        Some(20),
        Some(25),
    );

    write_event(connection, new_event);
    state.load_events(Some(connection));

    // nothing is selected by default
    test_state.select(None);
    assert_eq!(state.get_selection(), Some(&mut test_state));

    // incrementing the selection should point to the first element
    state.increment_selection();
    test_state.select(Some(0));
    assert_eq!(state.get_selection(), Some(&mut test_state));
}

#[test]
fn test_increment_selection() {
    std::env::set_var("DATABASE_URL", ":memory:");
    let mut state = terminal::state::AppState::initialized();
    let mut test_state = ListState::default();
    let connection: &mut SqliteConnection = &mut establish_connection();

    common::run_migrations(connection).expect("Error running migrations");

    let new_events: Vec<NewBabyEvent> = vec![
        create_event(
            Some(true),
            Some(true),
            Some(5),
            Some(10),
            Some(15),
            Some(20),
            Some(25),
        ),
        create_event(
            Some(true),
            Some(true),
            Some(5),
            Some(10),
            Some(15),
            Some(20),
            Some(25),
        ),
    ];

    write_event(connection, new_events[0]);
    state.load_events(Some(connection));

    // nothing is selected by default
    test_state.select(None);
    assert_eq!(state.get_selection(), Some(&mut test_state));

    // incrementing the selection should point to the first element
    state.increment_selection();
    test_state.select(Some(0));
    assert_eq!(state.get_selection(), Some(&mut test_state));

    // incrementing the selection in a vector of length 1 should not change the selection
    state.increment_selection();
    assert_eq!(state.get_selection(), Some(&mut test_state));

    // add another event
    write_event(connection, new_events[1]);
    state.load_events(Some(connection));

    // reloading events should reset the selection
    state.increment_selection();
    test_state.select(Some(0));
    assert_eq!(state.get_selection(), Some(&mut test_state));

    // incrementing the selection should point to the second element
    state.increment_selection();
    test_state.select(Some(1));
    assert_eq!(state.get_selection(), Some(&mut test_state));
}

#[test]
fn test_decrement_selection() {
    std::env::set_var("DATABASE_URL", ":memory:");
    let mut state = terminal::state::AppState::initialized();
    let mut test_state = ListState::default();
    let connection: &mut SqliteConnection = &mut establish_connection();

    common::run_migrations(connection).expect("Error running migrations");

    let new_events: Vec<NewBabyEvent> = vec![
        create_event(
            Some(true),
            Some(true),
            Some(5),
            Some(10),
            Some(15),
            Some(20),
            Some(25),
        ),
        create_event(
            Some(true),
            Some(true),
            Some(5),
            Some(10),
            Some(15),
            Some(20),
            Some(25),
        ),
    ];

    write_event(connection, new_events[0]);
    state.load_events(Some(connection));

    // nothing is selected by default
    test_state.select(None);
    assert_eq!(state.get_selection(), Some(&mut test_state));

    // decrementing the selection should point to the first element
    state.decrement_selection();
    test_state.select(Some(0));
    assert_eq!(state.get_selection(), Some(&mut test_state));

    // decrementing the selection in a vector of length 1 should not change the selection
    state.decrement_selection();
    test_state.select(Some(0));
    assert_eq!(state.get_selection(), Some(&mut test_state));

    // add another event
    write_event(connection, new_events[1]);
    state.load_events(Some(connection));

    // reloading events should reset the selection
    state.decrement_selection();
    test_state.select(Some(0));
    assert_eq!(state.get_selection(), Some(&mut test_state));

    // decrementing the selection should point to the last element
    state.decrement_selection();
    test_state.select(Some(1));
    assert_eq!(state.get_selection(), Some(&mut test_state));
}

#[test]
fn test_unselect() {
    std::env::set_var("DATABASE_URL", ":memory:");
    let mut state = terminal::state::AppState::initialized();
    let mut test_state = ListState::default();
    let connection: &mut SqliteConnection = &mut establish_connection();

    common::run_migrations(connection).expect("Error running migrations");

    let new_event: NewBabyEvent = create_event(
        Some(true),
        Some(true),
        Some(5),
        Some(10),
        Some(15),
        Some(20),
        Some(25),
    );

    write_event(connection, new_event);
    state.load_events(Some(connection));

    // nothing is selected by default
    test_state.select(None);
    assert_eq!(state.get_selection(), Some(&mut test_state));

    // incrementing the selection should point to the first element
    state.increment_selection();
    test_state.select(Some(0));
    assert_eq!(state.get_selection(), Some(&mut test_state));

    // unselecting should set the selection to None
    state.unselect();
    test_state.select(None);
    assert_eq!(state.get_selection(), Some(&mut test_state));
}
