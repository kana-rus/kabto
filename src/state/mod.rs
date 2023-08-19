use std::{sync::{OnceLock, Mutex}, marker::PhantomData};
use serde::{Serialize, Deserialize};


pub(crate) static STATES: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

pub fn use_state<T: Serialize + for<'d>Deserialize<'d>>(default: T) -> (State<T>, SetState<T>) {
    let mut states = STATES
        .get() .expect("")
        .lock().expect("");

    let id = states.len();
    states.push(serde_json::to_string(&default).expect(""));

    (
        State    { id, value_type: PhantomData },
        SetState { id, value_type: PhantomData }
    )
}

pub struct State<T: Serialize + for<'d>Deserialize<'d>> {
    id:         usize,
    value_type: PhantomData<fn()->T>,
} impl<T: Serialize + for<'d>Deserialize<'d>> Fn<()> for State<T> {
    extern "rust-call" fn call(&self, _: ()) -> Self::Output {
        let states = STATES
            .get() .expect("")
            .lock().expect("");
        serde_json::from_str(&states[self.id]).expect("")
    }
} const _: (/* by */) = {
    impl<T: Serialize + for<'d>Deserialize<'d>> FnOnce<()> for State<T> {
        type Output = T;
        extern "rust-call" fn call_once(self, _: ()) -> Self::Output {
            self.call(())
        }
    }
    impl<T: Serialize + for<'d>Deserialize<'d>> FnMut<()> for State<T> {
        extern "rust-call" fn call_mut(&mut self, _: ()) -> Self::Output {
            self.call(())
        }
    }
};

pub struct SetState<T: Serialize + for<'d>Deserialize<'d>> {
    id:         usize,
    value_type: PhantomData<fn()->T>,
} impl<T: Serialize + for<'d>Deserialize<'d>> Fn<(fn(T)->T,)> for SetState<T> {
    extern "rust-call" fn call(&self, (set,): (fn(T)->T,)) -> Self::Output {
        let mut states = STATES
            .get() .expect("")
            .lock().expect("");
        let t: T  = serde_json::from_str(&states[self.id]).expect("");
        let new_t = set(t);
        states[self.id] = serde_json::to_string(&new_t).expect("");
    }
} const _: (/* by */) = {
    impl<T: Serialize + for<'d>Deserialize<'d>> FnOnce<(fn(T)->T,)> for SetState<T> {
        type Output = ();
        extern "rust-call" fn call_once(self, (set,): (fn(T)->T,)) -> Self::Output {
            self.call((set,))
        }
    }
    impl<T: Serialize + for<'d>Deserialize<'d>> FnMut<(fn(T)->T,)> for SetState<T> {
        extern "rust-call" fn call_mut(&mut self, (set,): (fn(T)->T,)) -> Self::Output {
            self.call((set,))
        }
    }
};

