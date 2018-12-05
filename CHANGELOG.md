# __Next__
### New features
- New `InputBuilderOnce` `struct` to make the builder more flexible.
- Make methods take `impl ToString` rather than `&str` to improve flexibility.
### **Breaking changes**
- Module restructure.

# 0.6.1 and 0.6.2
### Documentation updates
- fix broken links

# 0.6.0
### New features
- Added `with_description`.
### Examples updates
- Add `match_num_err` example.
### Other
- Change fall back error message slightly. 

# 0.5.4
### Examples updates
- Add URL example

# 0.5.3
- Fix tiny error in readme
- Make examples downloadable
- Small internal changes

# 0.5.2
### Documentation updates
- fix tiny error in readme

# 0.5.1
### Documentation updates
- add information about using `match` with input.
- fix version number stated in readme.
### Examples updates
- Added `match` example

# 0.5.0
### New features
- Added `.repeat_msg()`.
### Documentation updates
- Document new features
- Make point_input example use new feature

# 0.4.5
### Documentation updates
- Make things more clear in readme

# 0.4.4
### Documentation updates
- Say a little more about type annotations in readme.
- fix inaccuracy in readme

# 0.4.3
### Documentation updates
- Add keywords and categories to `cargo.toml`

# 0.4.2
### Documentation updates
- Fix incorrect formatting in readme
- Add API docs to the code

# 0.4.1
### Documentation updates
- Fix incorrect version in readme tutorial

# __0.4.0__
### **Breaking changes**
- The `.test()` method has been made private. You will now need to use `.add_test()` and `.add_err_test()`. These are documented in readme.
- Closures are now taken without having to be behind a `&`. This will affects `.add_test()` and `.add_err_test()`
### New features
- Added `err_match()` method for custom errors
### Examples updates
- Added `point_input` example
### Documentation updates
- Remove the word simple from readme. `read_input` is not just for simple programs
- Add `point_input` to examples list in readme
- Document `err_match()` in readme
- Restructure readme
- Add missing `;`s to readme examples

# 0.3.8
### Examples updates
- Added chrono example
### Documentation updates
- place read_input in `` in readme
- List example programs in readme

# 0.3.7
### Documentation updates
- Note crates that implement `std::str::FromStr` in readme

# 0.3.6
### Documentation updates
- Make readme more clear.

# 0.3.5
### Documentation updates
- Make readme more clear.

# 0.3.4
### Documentation updates
- Document simple_input() and valid_input() in readme.md

# 0.3.3
### Documentation updates
- Update readme.

# 0.3.2
### Examples updates
- Added simple guessing game example.

# 0.3.1
### Examples updates
- Added guessing game example.

# 0.3.0
- Started logging changes.