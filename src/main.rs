use std::clone::Clone;
use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

impl<T, V: Clone + Copy + Eq + Hash> Cacher<T, V>
where
    T: Fn(V) -> V,
{
    fn new(calculation: T) -> Cacher<T, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: V) -> V {
        match self.values.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v.clone()
            }
        }
    }
}

struct Cacher<T, V>
where
    T: Fn(V) -> V,
{
    calculation: T,
    values: HashMap<V, V>,
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity + 1));
        println!("Next, do {} pullups!", expensive_result.value(intensity + 1));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}