use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::sync::Arc;
use tokio::sync::RwLock;

type Counters = HashMap<Arc<str>, isize>;
lazy_static! {
	static ref COUNTERS: RwLock<Counters> = RwLock::new(HashMap::new());
}

pub async fn increase(counter: Arc<str>) -> isize {
	let mut counters = COUNTERS.write().await;
	let entry = counters.entry(counter);

	match entry {
		Occupied(e) => {
			let e = e.into_mut();
			*e += 1;
			*e
		}
		Vacant(e) => {
			e.insert(1);
			1
		}
	}
}

pub async fn get(counter: Arc<str>) -> isize {
	let counters = COUNTERS.read().await;
	*counters.get(&counter).unwrap_or(&0)
}

pub async fn decrease(counter: Arc<str>) -> isize {
	let mut counters = COUNTERS.write().await;
	let entry = counters.entry(counter);
	match entry {
		Occupied(e) => {
			let e = e.into_mut();
			*e -= 1;
			*e
		}
		Vacant(e) => {
			e.insert(-1);
			-1
		}
	}
}

pub async fn set(counter: Arc<str>, value: isize) {
	let mut counters = COUNTERS.write().await;
	let entry = counters.entry(counter);
	match entry {
		Occupied(e) => *e.into_mut() = value,
		Vacant(e) => {
			e.insert(value);
		}
	};
}

pub async fn remove(counter: Arc<str>) {
	let mut counters = COUNTERS.write().await;
	_ = counters.remove(&counter);
}
