use std::{collections::HashSet, slice::SliceIndex};

use serde::{Deserialize, Serialize};

pub enum AnalysisErr {
    Redefinition {
        of: String
    },

    AltLabels
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateRef(usize);

#[derive(Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionRef(usize);

#[derive(Serialize, Deserialize)]
pub struct State {
    transitions: HashSet<TransitionRef>
}

impl State {
    pub fn transitions(&self) -> &HashSet<TransitionRef> {
        &self.transitions
    }
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct ATNFragment {
    start_state: StateRef,
    states: Vec<State>,
    transitions: Vec<Transition>
}

impl ATNFragment {
    pub fn append_fragment(&mut self, from: StateRef, to: ATNFragment) {
        
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