# Advent Of Code 2022, Day 11.

## Puzzle description
The full description of the puzzle can be found here: https://adventofcode.com/2022/day/11.

In short, you are given a set of monkeys, and each of them has some items. An item is just an integer number. Monkeys transform the items and then pass them to other monkeys based on rules.

Every monkey takes its turn and works on every item it has.

```
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
```

This monkey will take every item, multiply it by 19 and then check if it is divisible by 23. Based on the result it will pass it to Monkey 2 or Monkey 3.

## Part 1

In Part 1 of the puzzle, monkeys will divide items by 3 before examining if it is divisible.

Solving Part 1 was straightforward. Just one interesting thing about Rust ownership and borrowing.
```rust
// that's a pseudocode
fn make_turn(monkeys, name) {
    let items = std::mem::take(&mut monkeys[name].items); // moving of the items into local variable
    for mut item in items {
        ... // transform item based on monkeys[name] rules
        monkeys[dst_monkey].items.push(item);
    }
    Ok(())
}
```
This function performs all needed actions for all items of the given monkey. In this function, I take items from the current monkey into a local variable and replace them with the empty list, and then iterate over them. This is done by the `std::mem::take` function. At first, I tried to write it more straightforwardly, like this: `for item in monkeys[name].items`. But this did not work because it is not possible to modify any monkey while iterating over items of one of them, because this will mean holding an immutable reference to a whole list of monkeys.

That's interesting because this prevents a lot of possible bugs when modifying a list while iterating over it. For example, in this puzzle, it is possible to have a rule that will make the monkey pass an item to itself and this potentially can lead to an infinite loop, but rust forced me to avoid such a situation. And I can remember myself solving such bugs many times.

## Part 2

In part 2 the rule to divide items by 3 before inspecting is removed and the number of rounds to be performed is increased dramatically. That means that the values of the items can become large enough to not fit a 64-bit int. 

The first thing I thought about after reading the part 2 rules is that we only need to know if the item is divisible by a closed set of numbers: all distinct divisors specified by monkey rules. And also I remembered that there is such a thing as modular arithmetics. 

In short, that means that `(a + b) % d == a % d + b % d` and `(a * b) % d == a % d * b % d`. 

More about this: [Wikipedia](https://en.wikipedia.org/wiki/Modular_arithmetic).

And I decided to store each item not as its value, but as a set of its values modulo each possible divisor.

That worked and this solution can be found in [this revision](https://github.com/romamik/aoc2022/blob/d82a741feb486a487fd99d63781939ee15da5045/src/day11/mod.rs).

There is also one interesting thing about rust implementation. With items not being simple values anymore, it was desirable to avoid copies of the data structure used to hold item values. And with rust that was easy to implement. The code from Part 1 was already doing this.

### Part 2 improvement

Later I was thinking about my solution to part 2 and found out that it is not actually effective. Instead of holding distinct values for each possible divisor, it is possible to hold one value for an item modulo [least common multiple](https://en.wikipedia.org/wiki/Least_common_multiple) of all possible divisors. 

This significantly improved performance.