# Camera synchronization example

We have two cameras streaming images directly into the hardware.
We need to synchronize the cameras so that they are aligned in time as good as possible.

This leads us to a curious lifetime issue, which so far I have not been able to resolve without using `-Zpolonius`.

Thread on rust discussion forum here: https://users.rust-lang.org/t/curious-bororow-checker-problem-getting-image-buffers-from-two-camera-streams/119431

**Note** This only compiles if you use `export RUSTFLAGS="-Zpolonius"` in your shell.