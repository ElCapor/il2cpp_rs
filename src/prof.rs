use std::time::Instant;

pub struct ScopeTimer {
    label: &'static str,
    start: Instant,
}

impl ScopeTimer {
    pub fn new(label: &'static str) -> Self {
        Self { label, start: Instant::now() }
    }
    pub fn elapsed(&self) -> std::time::Duration {
        self.start.elapsed()
    }
}

impl Drop for ScopeTimer {
    fn drop(&mut self) {
        println!("{} took {:?}", self.label, self.start.elapsed());
    }
}

#[macro_export]
macro_rules! profile_scope {
    ($label:expr) => {
        let _profile_scope_timer = $crate::prof::ScopeTimer::new($label);
    };
}

#[macro_export]
macro_rules! profile_call {
    ($label:expr, $expr:expr) => {{
        let __start = std::time::Instant::now();
        let __ret = { $expr };
        println!("{} took {:?}", $label, __start.elapsed());
        __ret
    }};
}