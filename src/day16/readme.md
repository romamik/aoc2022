# Advent Of Code 2022, Day 16.

## Puzzle description
The full description of the puzzle can be found here: https://adventofcode.com/2022/day/15.

In this puzzle, we have the rooms connected with tunnels and a valve in each room. Each valve has its flow rate. It takes 1 minute to travel one tunnel and one minute to open a valve. We should maximize the total output of the valves, where the output of one valve is the time it has been open multiplied by its flow rate.

## Part 1

This seems like an NP-complete problem, somewhat similar to the traveling salesman problem. The naive greedy approach where we just select the next valve that will give maximum total output in the remaining time should not work, because, for example, it can be more profitable to open another two valves instead of this one.

Looking at the puzzle input we see there are a lot of valves with zero flow rates. Maybe converting them to direct connections between rooms with non-zero valves will reduce our graph and it will be possible to just check all possible orders of valves. In my input there like 15 non-zero valves, which gives us ~10^12 possible combinations, which is too much. But looking at the resulting graph I can see that most of these combinations will take longer than 30 minutes to travel, so I decided to test all possible visit orders that will take no longer than 30 minutes. That worked, but if the program is running for too long: like 5 seconds on my machine. 