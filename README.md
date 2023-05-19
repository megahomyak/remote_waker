[Available on crates.io!](https://crates.io/crates/remote_waker)

# Remote waker

This thing allows to wake a remote task. That's it. Helps, for example, when one task is pulling something from a mutexed queue and another task is putting something into the same queue. By utilizing the remote waker in such case, we can wake the first task only when the second task puts an item into the queue, which improves performance.

# Usage

Use a `new` function to get a `Waker` and a `Snoozer`, give the `Waker` to the waking task and the snoozer to the snoozing task. When applying to the example above, we'll need to give the `Waker` to the pushing task and the `Snoozer` to the pulling task.
