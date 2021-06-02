use std::os::raw::{c_char, c_int};

use isolate_sys::*;

#[cfg(test)]
mod tests;

pub struct IsolatedContainer {
    config: isolate_config
}

impl IsolatedContainer {
    pub fn run<T: Into<String>>(&self, command: Vec<T>) {
        let argv = command
            .into_iter()
            .map(|s| to_c_char(s.into()))
            .collect::<Vec<*mut c_char>>()
            .as_mut_ptr();

        let mut config = self.config.clone(); // TODO: Prevent clone
        config.argv = argv; // TODO: Is this useful ?

        unsafe {
            // TODO: Result ?
            run(argv, config.clone());
        }
    }

    pub fn delete(self) {
        unsafe {
            cleanup(self.config);
        }
    }
}

pub struct IsolatedContainerBuilder {
    id: usize,
    working_directory: String, // TODO: How to default?
    add_default_dirs: bool,
    inherit_environment: bool,
    files_size_limit: usize,
    stack_limit: usize,
    stdin: String,
    memory_limit: usize,
    disk_quota_blk: usize,
    disk_quota_ino: usize,
    redirect_stderr_to_stdout: bool,
    silent: bool,
    timeout: usize, // TODO: How to use fractions?
    verbose: bool, // TODO: How to use multiple times?
    wall_clock_timeout: usize,
    kill_delay_after_timeout: usize,
    share_network: bool,
    inherit_file_descriptors: bool
}

impl IsolatedContainerBuilder {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            working_directory: String::new(), // TODO: How to default?
            add_default_dirs: true,
            inherit_environment: false,
            files_size_limit: 0,
            stack_limit: 0,
            stdin: String::new(),
            memory_limit: 0,
            disk_quota_blk: 0,
            disk_quota_ino: 0,
            redirect_stderr_to_stdout: false,
            silent: false,
            timeout: 0,
            verbose: false,
            wall_clock_timeout: 0,
            kill_delay_after_timeout: 0,
            share_network: false,
            inherit_file_descriptors: false
        }
    }

    pub fn working_directory(mut self, working_directory: String) -> Self {
        self.working_directory = working_directory;
        self
    }

    pub fn add_default_dirs(mut self, add_default_dirs: bool) -> Self {
        self.add_default_dirs = add_default_dirs;
        self
    }

    pub fn inherit_environment(mut self, inherit_environment: bool) -> Self {
        self.inherit_environment = inherit_environment;
        self
    }

    pub fn files_size_limit(mut self, files_size_limit: usize) -> Self {
        self.files_size_limit = files_size_limit;
        self
    }

    pub fn stack_limit(mut self, stack_limit: usize) -> Self {
        self.stack_limit = stack_limit;
        self
    }

    pub fn stdin(mut self, stdin: String) -> Self {
        self.stdin = stdin;
        self
    }

    pub fn memory_limit(mut self, memory_limit: usize) -> Self {
        self.memory_limit = memory_limit;
        self
    }

    pub fn disk_quota(mut self, disk_quota_blk: usize, disk_quota_ino: usize) -> Self {
        self.disk_quota_blk = disk_quota_blk;
        self.disk_quota_ino = disk_quota_ino;

        self
    }

    pub fn redirect_stderr_to_stdout(mut self, redirect_stderr_to_stdout: bool) -> Self {
        self.redirect_stderr_to_stdout = redirect_stderr_to_stdout;
        self
    }

    pub fn silent(mut self, silent: bool) -> Self {
        self.silent = silent;
        self
    }

    pub fn timeout(mut self, timeout: usize) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn wall_clock_timeout(mut self, wall_clock_timeout: usize) -> Self {
        self.wall_clock_timeout = wall_clock_timeout;
        self
    }

    pub fn kill_delay_after_timeout(mut self, kill_delay_after_timeout: usize) -> Self {
        self.kill_delay_after_timeout = kill_delay_after_timeout;
        self
    }

    pub fn share_network(mut self, share_network: bool) -> Self {
        self.share_network = share_network;
        self
    }

    pub fn inherit_file_descriptors(mut self, inherit_file_descriptors: bool) -> Self {
        self.inherit_file_descriptors = inherit_file_descriptors;
        self
    }

    pub fn build(self) -> IsolatedContainer {
        let config = isolate_config {
            argv: std::ptr::null_mut(),
            box_id: self.id as c_int,
            set_cwd: to_c_char(self.working_directory),
            cg_enable: false.into(),
            default_dirs: self.add_default_dirs.into(),
            pass_environ: self.inherit_environment as c_int,
            fsize_limit: self.files_size_limit as c_int,
            stack_limit: self.stack_limit as c_int,
            redir_stdin: to_c_char(self.stdin),
            redir_stdout: to_c_char(String::new()), // TODO: How to use that?
            memory_limit: self.memory_limit as c_int,
            meta: to_c_char(String::new()), // TODO: How to use that?
            max_processes: 0 as c_int,
            blk: self.disk_quota_blk as c_int,
            ino: self.disk_quota_ino as c_int,
            redir_stderr: to_c_char(String::new()), // TODO: How to use that?
            redir_stderr_stdout: self.redirect_stderr_to_stdout.into(),
            silent: self.silent.into(),
            timeout: self.timeout as c_int,
            verbose: self.verbose.into(),
            wall_timeout: self.wall_clock_timeout as c_int,
            extra_timeout: self.kill_delay_after_timeout as c_int,
            cg_memory_limit: 0 as c_int,
            cg_timing: false.into(),
            share_net: self.share_network.into(),
            inherit_fds: self.inherit_file_descriptors.into()
        };

        unsafe {
            init(config.clone()); // TODO: Prevent clone
        }

        IsolatedContainer { config }
    }
}

fn to_c_char(str: String) -> *mut c_char {
    let mut bytes = str.bytes().map(|c| c as i8).collect::<Vec<i8>>();
    bytes.push(0);

    // TODO: Manage lifetime
    bytes.as_mut_ptr()
}