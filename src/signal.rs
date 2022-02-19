use std::{
    borrow::BorrowMut,
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    error::Error,
    ffi::{CString, CStr},
    hash::{Hash, Hasher},
    println,
    sync::{Arc, Mutex},
};

#[derive(Hash, PartialEq, Eq)]
pub enum Signal {
    /// Hangup detected on controlling terminal or death of controlling process.
    Hangup,
    /// Interrupt from keyboard.
    Interrupt,
    /// Quit from keyboard.
    Quit,
    /// Illegal instruction.
    Illegal,
    /// Trace/breakpoint trap.
    Trap,
    /// Abort signal from C abort function.
    Abort,
    /// IOT trap. A synonym for SIGABRT.
    IOT,
    /// Bus error (bad memory access).
    Bus,
    /// Floating point exception.
    FloatingPointException,
    /// Kill signal.
    Kill,
    /// User-defined signal 1.
    User1,
    /// Invalid memory reference.
    Segv,
    /// User-defined signal 2.
    User2,
    /// Broken pipe: write to pipe with no readers.
    Pipe,
    /// Timer signal from C alarm function.
    Alarm,
    /// Termination signal.
    Term,
    /// Child stopped or terminated.
    Child,
    /// Continue if stopped.
    Continue,
    /// Stop process.
    Stop,
    /// Stop typed at terminal.
    TSTP,
    /// Terminal input for background process.
    TTIN,
    /// Terminal output for background process.
    TTOU,
    /// Urgent condition on socket.
    Urgent,
    /// CPU time limit exceeded.
    XCPU,
    /// File size limit exceeded.
    XFSZ,
    /// Virtual alarm clock.
    VirtualAlarm,
    /// Profiling time expired.
    Profiling,
    /// Windows resize signal.
    Winch,
    /// I/O now possible.
    IO,
    /// Pollable event (Sys V). Synonym for IO
    Poll,
    /// Power failure (System V).
    ///
    /// Doesn't exist on apple systems so will be ignored.
    Power,
    /// Bad argument to routine (SVr4).
    Sys,
}

pub type SigNum = libc::c_int;
const SIG_NUM_MAX: SigNum = 64;

#[derive(Hash, PartialEq, Eq)]
struct SignalDescription {
    signal: Signal,
    description: String,
}

use crate::interner::*;



// lazy_static! {
//     static ref SIGNAL_DESCRIPTION_INTERNER: HashSetInterner<SignalDescription> =
//         HashSetInterner::new();
// }

// impl Internable for SignalDescription {
//     // type Hasher = DefaultHasher;
//     type Interner = HashSetInterner<SignalDescription>;
//     // type Key = Signal;

//     fn intern(value: Self) -> Interned<Self> {
//         SIGNAL_DESCRIPTION_INTERNER.intern(value)
//     }
// }

//static mut SIG_NUM_STR: [Option<&'static str>; SIG_NUM_MAX as usize] = [None; SIG_NUM_MAX as usize];
use crate::interner::intern;
impl Signal {
    // get a description from the os
    pub fn description(&self) -> Result<Interned<String>, Box<dyn Error>> {
        let sig_num = self.num();

        let description = unsafe {
            let description = libc::strsignal(self.num());
            println!("{:#?}", description);
            CStr::from_ptr(description)
        };
        
        let description = description.to_str()?;
        let description = description.to_string();
        let description = intern(description);
        Ok(description)
    }

    /// get the system number for this signal
    pub fn num(&self) -> SigNum {
        match self {
            Signal::Hangup => libc::SIGHUP,
            Signal::Interrupt => libc::SIGINT,
            Signal::Quit => libc::SIGQUIT,
            Signal::Illegal => libc::SIGILL,
            Signal::Trap => libc::SIGTRAP,
            Signal::Abort => libc::SIGABRT,
            Signal::IOT => libc::SIGIOT,
            Signal::Bus => libc::SIGBUS,
            Signal::FloatingPointException => libc::SIGFPE,
            Signal::Kill => libc::SIGKILL,
            Signal::User1 => libc::SIGUSR1,
            Signal::Segv => libc::SIGSEGV,
            Signal::User2 => libc::SIGUSR2,
            Signal::Pipe => libc::SIGPIPE,
            Signal::Alarm => libc::SIGALRM,
            Signal::Term => libc::SIGTERM,
            Signal::Child => libc::SIGCHLD,
            Signal::Continue => libc::SIGCONT,
            Signal::Stop => libc::SIGSTOP,
            Signal::TSTP => libc::SIGTSTP,
            Signal::TTIN => libc::SIGTTIN,
            Signal::TTOU => libc::SIGTTOU,
            Signal::Urgent => libc::SIGURG,
            Signal::XCPU => libc::SIGXCPU,
            Signal::XFSZ => libc::SIGXFSZ,
            Signal::VirtualAlarm => libc::SIGVTALRM,
            Signal::Profiling => libc::SIGPROF,
            Signal::Winch => libc::SIGWINCH,
            Signal::IO => libc::SIGIO,
            Signal::Poll => libc::SIGPOLL,
            Signal::Power => libc::SIGPWR,
            Signal::Sys => libc::SIGSYS,
        }
    }
}
