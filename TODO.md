# TODO

## Short Term

- [x] Fix the Wikipedia code
  * Fixed and it works now
- [ ] Add other generators
  * About halfway there, only need Reddit, Imgur, and LastFM
- [x] Implement dev commands
* Marked as done for now. Will add more as needed, right now have: adding a dev, removing a dev, and interfacing with the YouTube database as things to add

## Mid Term

- [x] Split up the commands by file
  * Marked as done, but I feel there's more that could be done
- [x] Implement some form of database for by-server settings
  * Implemented MongoDB - No by-server settings will be added because we were originally going to just use it for prefix storage and that is no longer needed (slash commands)
- [ ] Add YouTube video caching for quicker retrieval
  * Needed for youtube-dl's slow speed and Discord timing out applications after 5 seconds

## Long Term

### Sharding
- [ ] Implement a rudimentary sharding platform
  * No knowledge on how this works but further knowledge of serenity should enable this
- [ ] Making sure any shard can access the database(s) equally
  * Ensures ~~per-server settings are preserved~~ equal access to storage, will cross that bridge when we get there
- [ ] Introduce a shared cache in some form to reduce API requests
  * Reduces the chances of being rate limited and potential reusability of data

<sub>Side Note: if it's something small like code refactoring or an argument change, put it in the file with `// TODO [what to do]`</sub>