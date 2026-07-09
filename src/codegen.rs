use std::{collections::HashSet, slice::SliceIndex};

use serde::{Deserialize, Serialize};


#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateRef(pub usize);
impl StateRef {
    pub fn offset(&mut self, by: usize) {
        self.0 += by;
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionRef(usize);
impl TransitionRef {
    pub fn offset(&mut self, by: usize) {
        self.0 += by;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    transitions: HashSet<TransitionRef>
}

impl State {
    pub fn new() -> State {
        State {
            transitions: HashSet::new()
        }
    }
    
    pub fn transitions(&self) -> &HashSet<TransitionRef> {
        &self.transitions
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Transition {
    Epsilon {
        source: StateRef,
        target: StateRef
    },
    Atom {
        source: StateRef,
        target: StateRef,
        input: usize,
    },
    Range {
        source: StateRef,
        target: StateRef,
        start: usize,
        stop: usize,
    },
    Set {
        source: StateRef,
        target: StateRef,
        set: HashSet<usize>,
    },
    NotSet {
        source: StateRef,
        target: StateRef,
        not_set: HashSet<usize>,
    },
}

impl Transition {
    pub fn offset(&mut self, by: usize) {
        let (source, target) = match self {
            Self::Atom { source, target, .. }
            | Self::Epsilon { source, target, .. }
            | Self::Range { source, target, .. }
            | Self::Set { source, target, .. }
            | Self::NotSet { source, target, .. } => (source, target)
        };

        source.offset(by);
        target.offset(by);
    }

    pub fn is_epsilon(&self) -> bool {
        matches!(
            self,
            // Self::Rule { .. }
            | Self::Epsilon { .. }
            // | Self::Action { .. }
            // | Self::Predicate { .. }
            // | Self::PrecedencePredicate { .. }
        )
    }

    pub fn source(&self) -> StateRef {
        match self {
            Self::Atom { source, .. }
            | Self::Epsilon { source, .. }
            | Self::Range { source, .. }
            | Self::Set { source, .. }
            | Self::NotSet { source, .. } => *source

        }
    }

    pub fn target(&self) -> StateRef {
        match self {
            Self::Atom { target, .. }
            | Self::Epsilon { target, .. }
            | Self::Range { target, .. }
            | Self::Set { target, .. }
            | Self::NotSet { target, .. } => *target
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ATNFragment {
    start_state: StateRef,
    states: Vec<State>,
    transitions: Vec<Transition>
}

impl ATNFragment {
    pub fn new() -> ATNFragment {
        let mut states = Vec::new();
        states.push(State::new());

        ATNFragment { start_state: StateRef(0), states, transitions: Vec::new() }
    }

    pub fn offset(&mut self, states_len: usize, transitions_len: usize) {
        self.start_state.offset(states_len);
        
        for state in &mut self.states {
            state.transitions = state.transitions.clone().into_iter().map(|mut t| {
                t.offset(transitions_len);
                t
            }).collect()
        }

        for transition in &mut self.transitions {
            transition.offset(states_len); // Because the transitions hold StateRefs not TransitionRefs
        }
    }

    pub fn push_state(&mut self, s: State) {
        self.states.push(s);
    }

    pub fn push_transition(&mut self, t: Transition) {
        self.transitions.push(t);
    }

    pub fn append_fragment(&mut self, from: StateRef, mut to: ATNFragment) {
        let states_len = self.states.len();
        let transition_len = self.transitions.len();

        to.offset(states_len, transition_len);
        
        for state in to.states {
            self.states.push(state);
        }

        for transition in to.transitions {
            self.transitions.push(transition)
        }

        self.transitions.push(Transition::Epsilon { source: from, target: to.start_state })
    }

    pub fn closure(&self, current: StateRef, input: usize) -> HashSet<StateRef> {
        let mut states = self.epsilon_closure(current);
        states.insert(current);

        let mut closure = HashSet::new();

        let mut transitions = HashSet::new();
        for s in states.iter() {
            if let Some(state) = self.states.get(s.0) {
                transitions = transitions.union(state.transitions()).copied().collect()
            }
        }

        for t in transitions {
            if let Some(transition) = self.transitions.get(t.0) {
                let target = transition.target();

                match transition {
                    Transition::Atom { input: i, .. } => {
                        if input == *i {
                            closure.insert(target);
                        }
                    }

                    Transition::Range { start, stop, .. } => {
                        if input > *start && input < *stop {
                            closure.insert(target);
                        }
                    },
                    _ => ()
                }
            }
        }

        // closure = closure.union(other).collect();

        closure
    }

    pub fn epsilon_closure(&self, current: StateRef) -> HashSet<StateRef> {
        let mut closure = HashSet::new();

        if let Some(state) = self.states.get(current.0) {
            for t in state.transitions() {
                if let Some(transition) = self.transitions.get(t.0) {
                    if transition.is_epsilon() {
                        let mut sub_closure = self.epsilon_closure(transition.target());

                        closure.insert(transition.target());
                        closure = closure.union(&mut sub_closure).copied().collect();
                    }
                }
            }
        }

        closure
    }
}