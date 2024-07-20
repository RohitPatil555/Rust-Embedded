
# MINI - OS

## Task List

Below is high level task list to build mini-os project.

- [x] Create and develop workspace for project
- [x] Add various CI tools to handle error in code.
    - [x] Integrate Clippy, geiger, cargo-next, mockall tools
    - [x] Integrate make target to run CI.
- [x] Build allocator logic as like Bucket allocator.
    - [x] Develop initialization logic and verify same.
    - [x] Add validation logic to verify sufficient memory to create block.
    - [x] Add check to confirm on free address within pool.
- [x] Build logging framework for mini-os
    - [x] develop various log level macros.
- [ ] Mini OS
    - [ ] Develop UART base logger.
    - [ ] Configure global memory pool.
    - [ ] Build Task and Executor logic for Async programming.
    - [ ] Build Mini-Os entry function and add sample example code.
