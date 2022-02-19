// #![no_std]

extern crate libc;
pub mod signal;
use signal::*;
pub mod interner;

pub type Pid = libc::pid_t;
//https://man7.org/linux/man-pages/man2/kill.2.html

// enum SignalError {
//     INVAL = libc::EINVAL,
//     PERM = libc::EPERM,
//     SRCH = libc::ESRCH,
// }

// fn get_signals() {
//     unsafe {
//         libc::strsignal(0);
//     }
// }

/// An enum representing signals on UNIX-like systems.

// fn signal(pid: Pid, sig: Signal) {
//     unsafe {
//         let ans = libc::kill(pid, sig);
//     }
// }

/*

#include "pthread_impl.h"
 #include "lock.h"

 int pthread_kill(pthread_t t, int sig)
 {
     int r;
     sigset_t set;
     /* Block not just app signals, but internal ones too, since
      * pthread_kill is used to implement pthread_cancel, which
      * must be async-cancel-safe. */
     __block_all_sigs(&set);
     LOCK(t->killlock);
     r = t->tid ? -__syscall(SYS_tkill, t->tid, sig)
         : (sig+0U >= _NSIG ? EINVAL : 0);
     UNLOCK(t->killlock);
     __restore_sigs(&set);
     return r;
 }

*/

#[cfg(test)]
mod tests {
    use crate::signal::Signal;

    #[test]
    fn it_works() {
        let nice = Signal::Abort.description().unwrap();
    }
}
