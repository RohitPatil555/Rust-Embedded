
# MINI - OS

## Task List

Below is high level task list to build mini-os project.

- [x] Create and develop workspace for project
- [x] Add various CI tools to handle error in code.
    - [x] Integrate Clippy, geiger, cargo-next, mockall tools
    - [x] Integrate make target to run CI.
    - [ ] Integrate spellcheck tool.
- [ ] Build allocator logic as like Bucket allocator.
    - [x] Develop initialization logic and verify same.
    - [ ] Add check to confirm on free address within pool.
    - [ ] Develop and design multiple pool management.
- [ ] Build logging framework for mini-os
    - [ ] develop various log level macros.
    - [ ] push log to UART
- [ ] Build Task and Executor logic for Async programming.
- [ ] Build Mini-Os entry function and add sample example code.
