<h1 align="center">Parser</h1>
<p align="center">
    Mingling's Features
</p>

---

## Enable Feature

`parser` is a feature provided by **Mingling**. You can enable it in the following way:

```toml
[dependencies]
mingling = { 
    version = "...", 
    features = ["parser"] 
}
```

## Intro

`parser` provides the ability to transform user input into structured data. Its core concept is **pick**.

The following demonstrates the parsing approach without using a `Picker`:

```rust
#[chain]
async fn parse_hello(prev: HelloEntry) -> NextProcess {
    let args = &*prev;
    let first = args.first().cloned().unwrap_or_else(|| "World".to_string());
    ParsedHello::new(first).to_render()
}
```

This is how it looks when using `Picker`:

```rust
#[chain]
async fn parse_hello(prev: HelloEntry) -> NextProcess {
    // Create Picker
    let picker = Picker::<ThisProgram>::new(prev.inner);

    // Extract the first argument from the Picker, 
    //   fallback to "World" if it doesn't exist
    let first = picker
        .pick_or((), "World".to_string())
        .unpack_directly().0;

    ParsedHello::new(first).to_render()
}
```

You might notice that using `Picker` can sometimes make statements more verbose, but this is only when parsing a small number of arguments. What if we complicate the scenario?

Suppose we want to design the following commands:

```bash
# Eat 1 apple weighing at least 20
fruit eat Apple --min-weight 20

# Eat 10 apples weighing at least 20
fruit eat Apple --min-weight 20 --count 10

# Eat 1 apple weighing between 10 and 20
fruit eat Apple --min-weight 10 --max-weight 20

# Eat 1 apple weighing between 20 and 10 (incorrect logic)
fruit eat Apple --min-weight 20 --max-weight 10

# When no specific fruit is specified, eat banana
fruit eat --count 5
```

For this complex scenario, the `Picker` comes into play! 

We first design the type `ParsedEatFruit`

```rust
#[derive(Debug, Default, Groupped)]
struct ParsedEatFruit {
    count: i16,
    weight_range: (i16, i16),
    fruit_type: Fruit,
}

#[derive(Debug, Default, EnumTag)]
enum Fruit {
    #[default]
    Banana,
    Apple,
    Orange,
}
```

Then create the basic binary program `fruit`

```rust
use mingling::{
    EnumTag, Groupped,
    macros::{chain, dispatcher, gen_program, r_println, renderer},
    marker::NextProcess,
    parser::PickableEnum,
};

#[tokio::main]
async fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(FruitEatCommand);
    program.exec().await;
}

dispatcher!("eat",
    FruitEatCommand => FruitEatEntry);

#[derive(Debug, Default, Groupped)]
struct ParsedEatFruit {
    count: i16,
    weight_range: (i16, i16),
    fruit_type: Fruit,
}

#[derive(Debug, Default, EnumTag)]
enum Fruit {
    #[default]
    Banana,
    Apple,
    Orange,
}

// Implement PickableEnum for Fruit to make it pickable
impl PickableEnum for Fruit {}

#[chain]
async fn parse_fruit_eat(prev: FruitEatEntry) -> NextProcess {
    // ...
}

#[renderer]
fn render_fruit_eat(prev: ParsedEatFruit) {
    let weight_str = match prev.weight_range {
        (min, max) if min == 0 && max > 0 => {
            format!("up to {}.", max)
        }
        (min, max) if min > 0 && max == 0 => {
            format!("at least {}.", min)
        }
        (min, max) if min > 0 && max > 0 && min != max => {
            format!("between {} and {}.", min, max)
        }
        (min, max) if min > 0 && max > 0 && min == max => {
            format!("exactly {}.", min)
        }
        _ => "unknown.".to_string(),
    };

    let fruit_type = if prev.count > 1 {
        format!("{}s", prev.fruit_type.enum_info().0)
    } else {
        prev.fruit_type.enum_info().0.to_string()
    };

    r_println!(
        "I ate {} {}, each weighing {}",
        prev.count,
        fruit_type,
        weight_str
    );
}

gen_program!();
```

Now focus on writing the logic for `parse_fruit_eat`:

> Review the business logic:
>
> 1 - The default fruit is Banana
>
> 2 - The default quantity is 1
>
> 3 - The default weight is (0, 0)
>
> 4 - When `max-weight` is less than `min-weight`, the business logic is in error

Before writing the code, define the error type `MinGreaterThanMax` and the related `Renderer`

```rust
pack!(MinGreaterThanMax = ());

#[renderer]
fn render_min_greater_than_max(_prev: MinGreaterThanMax) {
    r_println!("Error: min weight cannot be greater than max weight.");
}
```

Now start writing the logic:

```rust
#[chain]
async fn parse_fruit_eat(prev: FruitEatEntry) -> NextProcess {
    let picker = Picker::new(prev.inner);
    let mut min_weight: i16 = 0;
    let parsed = picker
        .pick_or(["--count", "-n"], 1)
        .pick::<i16>("--min-weight") // default: 0
        .after(|min| {
            // Copy `min` to external variable
            min_weight = min;
            min
        })
        .pick_or::<i16>("--max-weight", min_weight) // default: min_weight
        .after_or_route(|max| {
            // Check if `max` is valid
            if max < &min_weight {
                Err(MinGreaterThanMax::default())
            } else {
                Ok(max.clone())
            }
        })
        .pick(())
        // Since there's a possibility of being routed, 
        //   don't use `unpack_directly`
        .unpack(); 
 
    match parsed {
        Ok((count, min_weight, max_weight, fruit_type)) => {
            let parsed = ParsedEatFruit {
                count,
                weight_range: (min_weight, max_weight),
                fruit_type,
            };
 
            AnyOutput::new(parsed).route_renderer()
        }
        Err(route) => route.to_render(),
    }
}
```

Complete code:

```rust
use mingling::{
    AnyOutput, EnumTag, Groupped,
    macros::{chain, dispatcher, gen_program, pack, r_println, renderer},
    marker::NextProcess,
    parser::{PickableEnum, Picker},
};

#[tokio::main]
async fn main() {
    let mut program = ThisProgram::new();
    program.with_dispatcher(FruitEatCommand);
    program.exec().await;
}

dispatcher!("eat",
    FruitEatCommand => FruitEatEntry);

#[derive(Debug, Default, Groupped)]
struct ParsedEatFruit {
    count: i16,
    weight_range: (i16, i16),
    fruit_type: Fruit,
}

#[derive(Debug, Default, EnumTag)]
enum Fruit {
    #[default]
    Banana,
    Apple,
    Orange,
}

impl PickableEnum for Fruit {}

pack!(MinGreaterThanMax = ());

#[chain]
async fn parse_fruit_eat(prev: FruitEatEntry) -> NextProcess {
    let picker = Picker::new(prev.inner);
    let mut min_weight: i16 = 0;
    let parsed = picker
        .pick_or(["--count", "-n"], 1)
        .pick::<i16>("--min-weight") // default: 0
        .after(|min| {
            // Copy `min` to external variable
            min_weight = min;
            min
        })
        .pick_or::<i16>("--max-weight", min_weight) // default: min_weight
        .after_or_route(|max| {
            // Check if `max` is valid
            if max < &min_weight {
                Err(MinGreaterThanMax::default())
            } else {
                Ok(max.clone())
            }
        })
        .pick(())
        .unpack();

    match parsed {
        Ok((count, min_weight, max_weight, fruit_type)) => {
            let parsed = ParsedEatFruit {
                count,
                weight_range: (min_weight, max_weight),
                fruit_type,
            };

            AnyOutput::new(parsed).route_renderer()
        }
        Err(route) => route.to_render(),
    }
}

#[renderer]
fn render_min_greater_than_max(_prev: MinGreaterThanMax) {
    r_println!("Error: min weight cannot be greater than max weight.");
}

#[renderer]
fn render_fruit_eat(prev: ParsedEatFruit) {
    let weight_str = match prev.weight_range {
        (min, max) if min == 0 && max > 0 => {
            format!("up to {}.", max)
        }
        (min, max) if min > 0 && max == 0 => {
            format!("at least {}.", min)
        }
        (min, max) if min > 0 && max > 0 && min != max => {
            format!("between {} and {}.", min, max)
        }
        (min, max) if min > 0 && max > 0 && min == max => {
            format!("exactly {}.", min)
        }
        _ => "unknown.".to_string(),
    };

    let fruit_type = if prev.count > 1 {
        format!("{}s", prev.fruit_type.enum_info().0)
    } else {
        prev.fruit_type.enum_info().0.to_string()
    };

    r_println!(
        "I ate {} {}, each weighing {}",
        prev.count,
        fruit_type,
        weight_str
    );
}

gen_program!();
```

Now compile the program and run it:

```bash
cargo install --path ./
```

Running results:

```bash
~> fruit eat Apple --min-weight 20
I ate 1 Apple, each weighing exactly 20.

~> fruit eat Apple --min-weight 20 --count 10
I ate 10 Apples, each weighing exactly 20.

~> fruit eat Apple --min-weight 10 --max-weight 20
I ate 1 Apple, each weighing between 10 and 20.

~> fruit eat Apple --min-weight 20 --max-weight 10
Error: min weight cannot be greater than max weight.

~> fruit eat --count 5
I ate 5 Bananas, each weighing unknown.
```
