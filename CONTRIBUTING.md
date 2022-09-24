## Contributing

1. Fork the repository

2. Next, make a branch with a name related to the changes you're making (IE New Command)
1. This makes it easier to keep your main branch fork up-to-date

3. Once you're checked out to your new branch, you're going to want to have a block of code like:
```rust
/// A descriptive description
#[poise::command(slash_command)]
pub async fn command(
ctx: Context<'_>,
#[description = "Option 1"] option1: Option<T>,
    #[description = "Option 2"] option2: Option<T>,
        ) -> Result<(), Error> {
            // Code goes here
            Ok(())
            }
            ```

4. Go into the main function and look for a massive vector around lines `95-125`, you can't miss it because it takes up a lot of space. It should look like:
```rs
commands: vec![
...,
module::command(),
...
],
```

5. Add your command like `module::command()` (`module::` not needed if it's inside of `main.rs`) to the vector (don't forget a comma)

6. Submit a Pull Request stating the changes you made and wait for a maintainer to get back to you.
1. Do note, all three devs are college students and may be busy so please be patient.