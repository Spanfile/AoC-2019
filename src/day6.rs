use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug)]
struct Object {
    pub id: String,
    pub orbits: RefCell<Option<Rc<Object>>>,
}

impl Object {
    fn get_direct_orbits(&self) -> i32 {
        if self.orbits.borrow().is_some() {
            1
        } else {
            0
        }
    }

    fn get_indirect_orbits(&self) -> i32 {
        let orbits = self.orbits.borrow();
        if let Some(orbits) = orbits.as_ref() {
            orbits.get_direct_orbits() + orbits.get_indirect_orbits()
        } else {
            0
        }
    }

    fn get_path(&self) -> HashMap<String, i32> {
        let mut path = HashMap::new();

        if let Some(orbits) = self.orbits.borrow().as_ref() {
            path.insert(orbits.id.clone(), 0);
            for (id, len) in orbits.get_path() {
                path.insert(id, len + 1);
            }
        }

        path
    }
}

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<(String, String)> {
    input
        .split('\n')
        .map(|s| {
            let args: Vec<&str> = s.split(')').collect();
            (String::from(args[0]), String::from(args[1]))
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[(String, String)]) -> i32 {
    build_orbits(input)
        .values()
        .map(|o| o.get_direct_orbits() + o.get_indirect_orbits())
        .sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &[(String, String)]) -> i32 {
    let objects = build_orbits(input);
    let you_path = objects.get("YOU").unwrap().get_path();
    let san_path = objects.get("SAN").unwrap().get_path();
    let mut you_path_set = HashSet::new();
    let mut san_path_set = HashSet::new();

    for o in objects.get("YOU").unwrap().get_path() {
        you_path_set.insert(o.0);
    }

    for o in objects.get("SAN").unwrap().get_path() {
        san_path_set.insert(o.0);
    }

    let mut min_dist = std::i32::MAX;
    for i in you_path_set.intersection(&san_path_set) {
        let distance = you_path.get(i).unwrap() + san_path.get(i).unwrap();
        if distance < min_dist {
            min_dist = distance;
        }
    }

    min_dist
}

fn build_orbits(input: &[(String, String)]) -> HashMap<String, Rc<Object>> {
    let mut objects = HashMap::new();
    objects.insert(
        String::from("COM"),
        Rc::new(Object {
            id: String::from("COM"),
            orbits: RefCell::new(None),
        }),
    );

    for (orbits_id, object_id) in input {
        let orbits = if let Some(orbits) = objects.get(orbits_id) {
            Rc::clone(orbits)
        } else {
            let orbits = Rc::new(Object {
                id: String::from(orbits_id),
                orbits: RefCell::new(None),
            });
            objects.insert(orbits_id.to_owned(), Rc::clone(&orbits));
            orbits
        };

        if let Some(object) = objects.get_mut(object_id) {
            *object.orbits.borrow_mut() = Some(orbits);
        } else {
            objects.insert(
                String::from(object_id),
                Rc::new(Object {
                    id: String::from(object_id),
                    orbits: RefCell::new(Some(orbits)),
                }),
            );
        }
    }

    objects
}
