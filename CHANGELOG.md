# **0.4.0**
### **Breaking changes**
- The `.test()` method has been made private. You will now need to use `.add_test()` and `.add_err_test()`. These are documented in readme.
- Closures are now taken without having to be behind a `&`. This will affects `.add_test()` and `.add_err_test()`
### New features
- Added `err_match()` method for custom errors TODO doc this
### Examples updates
- Add `point_input` example
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
- List example programs to readme

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