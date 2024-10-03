use crate::trap;

// region TaskContext begin
#[repr(C)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {
    pub fn empty() -> Self {
        TaskContext {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }

    pub fn goto_restore(trap_cx: usize) -> Self {
        TaskContext {
            ra: trap::__restore_trap as usize,
            sp: trap_cx,
            s: [0; 12],
        }
    }
}
// region TaskContext end