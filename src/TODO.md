# TODO

## Short Term

- [x] Fix the Wikipedia code
  * Fixed and it works now
- [ ] Add other generators
  * About halfway there
- [ ] Implement dev commands (IE On-the-fly status changing)
  * Right now I don't know a way to set the status but further knowledge of serenity may allow that

## Mid Term

- [x] Split up the commands by file
  * Marked as done, but I feel there's more that could be done
- [ ] Implement some form of database for by-server settings
  * Right now MongoDB has a native driver for Rust so that is most likely, however MySQL or family may be chosen to gain experience that way

## Long Term

### Sharding
- [ ] Implement a rudimentary sharding platform
  * No knowledge on how this works but further knowledge of serenity should enable this
- [ ] Making sure any shard can access the database(s) equally
  * Ensures per-server settings are preserved, will cross that bridge when we get there
- [ ] Introduce a shared cache in some form to reduce API requests
  * Reduces the chances of being rate limited and potential reusability of data

<sub>Side Note: if it's something small like code refactoring or an argument change, put it in the file with `// TODO [what to do]`</sub>