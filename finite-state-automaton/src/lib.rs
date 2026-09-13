#[derive(PartialEq, Eq)]
enum State {
    Start,
    AfterA,
    Accept,
    Dead,
}

pub fn recognize_pattern(input: &str) -> bool {
    let mut state = State::Start;

    input.chars().for_each(|c| {
        state = next(&state, c);
    });

    state == State::Accept
}

fn next(state: &State, c: char) -> State {
    match (state, c) {
        (State::Start, 'a') => State::AfterA,
        (State::AfterA, 'b') => State::AfterA,
        (State::AfterA, 'c') => State::Accept,
        (_, _) => State::Dead,
    }
}
