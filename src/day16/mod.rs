mod parser;

use crate::solution::{Solution, SolutionInput};
use anyhow::{anyhow, bail, Result};

use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

use self::parser::parse_input;

#[derive(Debug, PartialEq, Eq)]
pub struct Room {
    flow: usize,
    tunnels: Vec<String>,
}

type Input = HashMap<String, Room>; // name => Room

fn find_travel_time(input: &Input, from: &str, to: &str) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((from, 0));
    visited.insert(from);

    while let Some((name, time)) = queue.pop_front() {
        if let Some(room) = input.get(name) {
            let next_time = time + 1;
            for next in room.tunnels.iter() {
                if next == to {
                    return Some(next_time);
                }

                if !visited.contains(next.as_str()) {
                    queue.push_back((next, next_time));
                    visited.insert(next);
                }
            }
        }
    }
    None
}

#[derive(Debug)]
struct Graph(HashMap<String, HashMap<String, usize>>); // room_name_from => (room_name_to => travel_time)

impl Graph {
    pub fn new() -> Graph {
        Graph(HashMap::new())
    }

    pub fn add_connection(&mut self, from: &str, to: &str, time: usize) {
        let mut add = |from: &str, to: &str| {
            self.0
                .entry(from.to_string())
                .or_default()
                .insert(to.to_string(), time)
        };

        add(from, to);
        add(to, from);
    }

    pub fn get_time(&self, from: &str, to: &str) -> Option<usize> {
        self.0.get(from).and_then(|it| it.get(to)).cloned()
    }

    pub fn get_rooms(&self) -> Vec<&str> {
        self.0.keys().map(|it| it.as_str()).collect_vec()
    }
}

fn make_graph_from_input(start_room: &str, input: &Input) -> Graph {
    let non_zero_rooms = input
        .iter()
        .filter(|(name, room)| room.flow != 0 || name.as_str() == start_room)
        .map(|(name, _)| name)
        .collect_vec();

    let mut graph = Graph::new();

    for (room_a, room_b) in non_zero_rooms.iter().tuple_combinations() {
        if let Some(time) = find_travel_time(input, room_a, room_b) {
            graph.add_connection(room_a, room_b, time)
        }
    }

    for room in non_zero_rooms.iter() {
        graph.add_connection(room, room, 0);
    }

    graph
}

fn calc_total_flow_for_visit_order(
    input: &Input,
    graph: &Graph,
    start_room: &str,
    order: &[&str],
    max_time: usize,
) -> Result<(usize, usize)> {
    let mut time_left = max_time;
    let mut total_flow = 0;

    let mut prev_room_name = start_room;

    for room_name in order.iter().cloned() {
        let travel_time = graph
            .get_time(prev_room_name, room_name)
            .ok_or_else(|| anyhow!("no way between {:?} and {:?}", prev_room_name, room_name))?;

        let room = input
            .get(room_name)
            .ok_or_else(|| anyhow!("no room {:?} in input", room_name))?;

        if (travel_time + 1) > time_left {
            bail!("out of time");
        }

        time_left -= travel_time + 1;
        total_flow += time_left * room.flow;

        prev_room_name = room_name;
    }

    Ok((total_flow, time_left))
}

impl SolutionInput for Input {
    fn parse(input_str: &str) -> Result<Self> {
        parse_input(input_str)
    }
}

fn for_all_visit_orders<F: FnMut(&[&str], usize) -> Result<()>>(
    start_room: &str,
    rooms: &[&str],
    input: &Input,
    graph: &Graph,
    max_time: usize,
    mut cb: F,
) -> Result<()> {
    // recursively visit all possible orders of visit
    fn gen<'a, F: FnMut(&[&str], usize) -> Result<()>>(
        prev_room: &'a str,
        vec: &mut Vec<&'a str>,
        rooms: &mut HashSet<&'a str>,
        input: &Input,
        graph: &Graph,
        time_left: usize,
        total_flow: usize,
        cb: &mut F,
    ) -> Result<()> {
        let mut traveled = false;
        for next_room in rooms.iter().cloned().collect_vec() {
            if let Some(travel_time) = graph.get_time(prev_room, next_room) {
                if (travel_time + 1) <= time_left {
                    rooms.remove(next_room);
                    vec.push(next_room);
                    let time_left = time_left - (travel_time + 1);

                    let next_room_flow = input
                        .get(next_room)
                        .ok_or_else(|| anyhow!("no room {:?}", next_room))?
                        .flow;
                    let total_flow = total_flow + next_room_flow * time_left;
                    gen(
                        next_room, vec, rooms, input, graph, time_left, total_flow, cb,
                    )?;

                    rooms.insert(next_room);
                    vec.pop();

                    traveled = true;
                }
            }
        }

        if !traveled {
            cb(vec, total_flow)?;
        }

        Ok(())
    }

    let mut rooms = HashSet::from_iter(rooms.iter().cloned());
    rooms.remove(start_room);

    // start the search
    gen(
        start_room,
        &mut Vec::new(),
        &mut rooms,
        input,
        graph,
        max_time,
        0,
        &mut cb,
    )?;

    Ok(())
}

pub struct Day16Pt1;
impl Solution for Day16Pt1 {
    const DAY: usize = 16;
    const PART: usize = 1;

    type TInput = Input;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        let start_room = "AA";
        let max_time = 30;
        let graph = make_graph_from_input(start_room, input);
        let rooms = graph.get_rooms();

        let mut max_flow = 0;

        for_all_visit_orders(
            start_room,
            &rooms,
            input,
            &graph,
            max_time,
            |_order, flow| {
                //let (flow, _time_left) =
                // calc_total_flow_for_visit_order(input, &graph, start_room, order, max_time)
                //     .with_context(|| anyhow!("visit order: {:?}", order))?;
                max_flow = max_flow.max(flow);
                Ok(())
            },
        )?;

        Ok(max_flow)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::get_input;
    use lazy_static::lazy_static;
    use ntest::timeout;

    lazy_static! {
        static ref INPUT_TEST: Input = get_input::<Day16Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: Input = get_input::<Day16Pt1>("input.txt").unwrap();
    }

    #[test]
    #[timeout(1000)]
    fn test_part1_result() -> Result<()> {
        assert_eq!(2080, Day16Pt1::solve(&INPUT_MAIN)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(1651, Day16Pt1::solve(&INPUT_TEST)?);
        Ok(())
    }

    #[test]
    fn test_calc_total_flow_for_visit_order() -> Result<()> {
        let input: &Input = &INPUT_TEST;
        let graph = make_graph_from_input("AA", input);
        let max_time = 30;

        assert_eq!(
            (364, 28),
            calc_total_flow_for_visit_order(input, &graph, "AA", &["BB"], max_time)?
        );

        assert_eq!(
            (1651, 6),
            calc_total_flow_for_visit_order(
                input,
                &graph,
                "AA",
                &["DD", "BB", "JJ", "HH", "EE", "CC"],
                max_time
            )?
        );
        Ok(())
    }

    #[test]
    fn test_make_graph_from_input() -> Result<()> {
        let input = parse_input(
            &[
                "Valve AA has flow rate=0; tunnels lead to valves BB",
                "Valve BB has flow rate=13; tunnels lead to valves AA, CC",
                "Valve CC has flow rate=0; tunnels lead to valves BB, DD",
                "Valve DD has flow rate=13; tunnels lead to valves CC",
            ]
            .join("\n"),
        )?;
        let graph = make_graph_from_input("AA", &input);

        assert_eq!(graph.get_time("AA", "AA"), Some(0)); // allow travel to self
        assert_eq!(graph.get_time("AA", "BB"), Some(1));
        assert_eq!(graph.get_time("AA", "CC"), None); // zero flow rooms excluded
        assert_eq!(graph.get_time("AA", "DD"), Some(3)); // but it is possible to go through them

        Ok(())
    }

    #[test]
    fn test_travel_time() -> Result<()> {
        let input = parse_input(
            &[
                "Valve AA has flow rate=0; tunnels lead to valves BB",
                "Valve BB has flow rate=13; tunnels lead to valves AA, CC, DD",
                "Valve CC has flow rate=0; tunnels lead to valves BB, DD",
                "Valve DD has flow rate=13; tunnels lead to valves CC",
            ]
            .join("\n"),
        )?;
        assert_eq!(Some(2), find_travel_time(&input, "AA", "DD"));
        Ok(())
    }
}
