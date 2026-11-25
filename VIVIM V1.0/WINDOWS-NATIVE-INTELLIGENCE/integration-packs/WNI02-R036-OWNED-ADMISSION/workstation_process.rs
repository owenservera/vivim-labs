//! Local native process lifecycle primitive for the existing Host.
//! Does not grant authority. A trusted Host must bind an approved executable,
//! arguments, workspace, environment and effect receipt before calling start.
//! No shell expansion, external PID control, network listener or remote tool.
use std::{
    collections::VecDeque,
    ffi::OsString,
    io::{self, Read, Write},
    path::PathBuf,
    process::{Child, Command, ExitStatus, Stdio},
    sync::{Arc, Mutex, mpsc::{self,SyncSender,TrySendError}},
    thread,
    time::{Duration, Instant},
};

#[cfg(windows)]
#[path="workstation_windows_job.rs"]
mod windows_job;
#[cfg(windows)]
use windows_job::Process as PlatformChild;
#[cfg(not(windows))]
use Child as PlatformChild;

const MAX_TAIL: usize = 65_536;
const MAX_WAIT: Duration = Duration::from_secs(120);
const MAX_ARGS: usize = 256;

pub struct LocalProcessSpec {
    pub executable: PathBuf,
    pub workspace: PathBuf,
    pub args: Vec<OsString>,
    /// Explicit, locally supplied environment only; never inherit credentials by default.
    pub environment: Vec<(OsString, OsString)>,
    pub tail_bytes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputTail {
    pub bytes: Vec<u8>,
    pub observed_bytes: u64,
    pub closed: bool,
    pub read_failed: bool,
}

struct Tail {
    bytes: VecDeque<u8>,
    observed_bytes: u64,
    limit: usize,
    closed: bool,
    read_failed: bool,
}
impl Tail {
    fn new(limit: usize) -> Self {
        Self { bytes: VecDeque::new(), observed_bytes: 0, limit, closed: false, read_failed: false }
    }
    fn push(&mut self, chunk: &[u8]) {
        self.observed_bytes = self.observed_bytes.saturating_add(chunk.len() as u64);
        for byte in chunk {
            if self.bytes.len() == self.limit { self.bytes.pop_front(); }
            self.bytes.push_back(*byte);
        }
    }
    fn snapshot(&self) -> OutputTail {
        OutputTail { bytes: self.bytes.iter().copied().collect(), observed_bytes: self.observed_bytes,
            closed: self.closed, read_failed: self.read_failed }
    }
}

fn invalid() -> io::Error { io::Error::new(io::ErrorKind::InvalidInput, "invalid local process specification") }
fn drain<R: Read + Send + 'static>(mut pipe: R, tail: Arc<Mutex<Tail>>) -> io::Result<()> {
    thread::Builder::new().name("vivim-process-output".into()).spawn(move || {
        let mut buf = [0u8; 4096];
        loop {
            match pipe.read(&mut buf) {
                Ok(0) => { if let Ok(mut state)=tail.lock() { state.closed=true; } break; }
                Ok(n) => { if let Ok(mut state)=tail.lock() { state.push(&buf[..n]); } else { break; } }
                Err(e) if e.kind()==io::ErrorKind::Interrupted => continue,
                Err(_) => { if let Ok(mut state)=tail.lock() { state.closed=true; state.read_failed=true; } break; }
            }
        }
    })?;
    Ok(())
}

/// An owned child handle, never a PID supplied by a caller. Output is local-only
/// until the Host projects it for the authorized audience. Interactive stdin
/// is explicitly enabled. Windows owns a creation-time job; other platforms own the direct child only.
pub struct OwnedProcess {
    child: PlatformChild,
    status: Option<ExitStatus>,
    stdout: Arc<Mutex<Tail>>,
    stderr: Arc<Mutex<Tail>>,
    input: Option<SyncSender<Vec<u8>>>,
}

impl OwnedProcess {
    pub fn start(spec: LocalProcessSpec) -> io::Result<Self> {Self::start_with_input(spec,false)}
    pub fn start_with_input(spec: LocalProcessSpec,interactive:bool) -> io::Result<Self> {
        if !spec.executable.is_absolute() || !spec.workspace.is_absolute() ||
            !(1..=MAX_TAIL).contains(&spec.tail_bytes) || spec.args.len()>MAX_ARGS ||
            spec.environment.len()>MAX_ARGS { return Err(invalid()); }
        let executable=spec.executable.canonicalize()?;
        let workspace=spec.workspace.canonicalize()?;
        if !executable.is_file() || !workspace.is_dir() { return Err(invalid()); }
        // On Windows .cmd/.bat may implicitly invoke a shell. Only native .exe.
        #[cfg(windows)]
        if !executable.extension().and_then(|s|s.to_str()).is_some_and(|s|s.eq_ignore_ascii_case("exe")) {
            return Err(invalid());
        }
        #[cfg(windows)]
        let (child,stdout,stderr,stdin)=PlatformChild::spawn(&executable,&workspace,&spec.args,&spec.environment,interactive)?;
        #[cfg(not(windows))]
        let (child,stdout,stderr,stdin)={
        let mut command=Command::new(executable);
        command.current_dir(workspace).args(spec.args).env_clear().envs(spec.environment)
            .stdin(if interactive {Stdio::piped()} else {Stdio::null()}).stdout(Stdio::piped()).stderr(Stdio::piped());
        let mut child=command.spawn()?;
        let stdout=child.stdout.take().ok_or_else(invalid)?;
        let stderr=child.stderr.take().ok_or_else(invalid)?;
        let stdin=child.stdin.take();
        (child,stdout,stderr,stdin)
        };
        let mut process=Self {child,status:None,
            stdout:Arc::new(Mutex::new(Tail::new(spec.tail_bytes))),
            stderr:Arc::new(Mutex::new(Tail::new(spec.tail_bytes))),input:None};
        // The owned guard cancels its child/job on a setup error as well.
        drain(stdout,Arc::clone(&process.stdout))?;
        drain(stderr,Arc::clone(&process.stderr))?;
        if interactive {
            let mut pipe=stdin.ok_or_else(invalid)?;
            let (tx,rx)=mpsc::sync_channel::<Vec<u8>>(8);
            thread::Builder::new().name("vivim-process-input".into()).spawn(move||{
                while let Ok(bytes)=rx.recv(){if pipe.write_all(&bytes).and_then(|_|pipe.flush()).is_err(){break;}}
            })?;
            process.input=Some(tx);
        }
        Ok(process)
    }
    /// Queue bounded input; accepted means queued, not consumed by the child.
    pub fn send_input(&mut self,bytes:Vec<u8>)->io::Result<()> {
        if bytes.is_empty()||bytes.len()>4096||self.poll()?.is_some(){return Err(invalid());}
        let tx=self.input.as_ref().ok_or_else(invalid)?;
        tx.try_send(bytes).map_err(|e|match e {TrySendError::Full(_)=>io::Error::new(io::ErrorKind::WouldBlock,"input queue full"),TrySendError::Disconnected(_)=>io::Error::new(io::ErrorKind::BrokenPipe,"input closed")})
    }
    pub fn close_input(&mut self){self.input.take();}
    pub fn poll(&mut self) -> io::Result<Option<ExitStatus>> {
        if self.status.is_none() { self.status=self.child.try_wait()?; }
        Ok(self.status)
    }
    /// Wait at most the caller's bounded interval. A timeout does not imply
    /// completion or silently launch a replacement.
    pub fn wait_for(&mut self, timeout: Duration) -> io::Result<Option<ExitStatus>> {
        if timeout>MAX_WAIT { return Err(invalid()); }
        let start=Instant::now();
        loop {
            if let Some(status)=self.poll()? { return Ok(Some(status)); }
            let Some(remaining)=timeout.checked_sub(start.elapsed()) else { return Ok(None); };
            if remaining.is_zero() { return Ok(None); }
            thread::sleep(remaining.min(Duration::from_millis(10)));
        }
    }
    /// Cancel the owned Windows job (or direct child on other platforms).
    pub fn cancel(&mut self) -> io::Result<ExitStatus> {
        if let Some(status)=self.poll()? { return Ok(status); }
        self.close_input();
        if let Err(error)=self.child.kill() {
            if let Some(status)=self.poll()? { return Ok(status); }
            return Err(error);
        }
        let status=self.child.wait()?;
        self.status=Some(status);
        Ok(status)
    }
    /// Two bounded byte tails; closure is separate from direct-child exit
    /// (for example, an inherited pipe can remain open in a descendant).
    pub fn output(&self) -> io::Result<(OutputTail,OutputTail)> {
        let stdout=self.stdout.lock().map_err(|_|io::Error::other("output unavailable"))?.snapshot();
        let stderr=self.stderr.lock().map_err(|_|io::Error::other("output unavailable"))?.snapshot();
        Ok((stdout,stderr))
    }
}
impl Drop for OwnedProcess {
    fn drop(&mut self) {
        self.close_input();
        if self.status.is_none() {
            match self.child.try_wait() {
                Ok(Some(_))=>{},
                Ok(None)|Err(_)=>{ if self.child.kill().is_ok() { let _=self.child.wait(); } }
            }
        }
        // Windows job-handle closure also kills remaining descendants, including
        // on worker crash. On other platforms detached readers may outlive the direct child.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    fn spec(mode:&str, cap:usize)->LocalProcessSpec {
        let mut environment=vec![("VIVIM_OWNED_CHILD_TEST".into(),mode.into())];
        if let Some(root)=std::env::var_os("SystemRoot") { environment.push(("SystemRoot".into(),root)); }
        LocalProcessSpec {executable:std::env::current_exe().unwrap(),
            workspace:std::env::current_dir().unwrap(),
            args:vec!["--exact".into(),"workstation_process::tests::fixture_child".into(),"--nocapture".into(),"--test-threads=1".into()],
            environment,tail_bytes:cap}
    }
    #[test]
    fn fixture_child() {
        match std::env::var("VIVIM_OWNED_CHILD_TEST").as_deref() {
            Ok("emit")=>{ println!("VIVIM_STDOUT_MARKER"); eprintln!("VIVIM_STDERR_MARKER"); }
            Ok("large")=>{ let mut out=std::io::stdout().lock(); out.write_all(&vec![b'x';131_072]).unwrap();out.flush().unwrap(); }
            Ok("sleep")=>thread::sleep(Duration::from_secs(20)),
            #[cfg(windows)]
            Ok("tree")=>{
                let mut spec=spec("descendant",128);
                let mut child=Command::new(&spec.executable);
                child.args(spec.args.drain(..)).env_clear().envs(spec.environment);
                let _child=child.spawn().unwrap();
                println!("VIVIM_ROOT_EXITING");
            }
            #[cfg(windows)]
            Ok("descendant")=>{println!("VIVIM_DESCENDANT_STARTED");thread::sleep(Duration::from_secs(20));}

            _=>{}
        }
    }
    fn closed_output(p:&OwnedProcess)->(OutputTail,OutputTail) {
        let start=Instant::now();
        loop {
            let (out,err)=p.output().unwrap();
            if out.closed&&err.closed {return (out,err);}
            assert!(start.elapsed()<Duration::from_secs(5),"output readers did not close");
            thread::sleep(Duration::from_millis(10));
        }
    }
    #[test]
    fn native_child_exit_and_both_streams() {
        let mut p=OwnedProcess::start(spec("emit",4096)).unwrap();
        assert!(p.wait_for(Duration::from_secs(10)).unwrap().unwrap().success());
        let (out,err)=closed_output(&p);
        assert!(String::from_utf8_lossy(&out.bytes).contains("VIVIM_STDOUT_MARKER"));
        assert!(String::from_utf8_lossy(&err.bytes).contains("VIVIM_STDERR_MARKER"));
        assert!(!out.read_failed&&!err.read_failed);
    }
    #[test]
    fn bounded_tail_drains_large_output() {
        let mut p=OwnedProcess::start(spec("large",128)).unwrap();
        assert!(p.wait_for(Duration::from_secs(10)).unwrap().unwrap().success());
        let (out,_)=closed_output(&p);
        assert!(out.observed_bytes>=131_072);
        assert_eq!(out.bytes.len(),128);
    }
    #[test]
    fn bounded_wait_and_owned_cancellation() {
        let mut p=OwnedProcess::start(spec("sleep",128)).unwrap();
        assert!(p.wait_for(Duration::from_millis(20)).unwrap().is_none());
        let status=p.cancel().unwrap();
        assert!(!status.success());
        assert_eq!(p.poll().unwrap(),Some(status));
        assert_eq!(p.cancel().unwrap(),status);
        closed_output(&p);
    }
    #[cfg(windows)]
    #[test]
    fn root_exit_does_not_complete_job_and_cancel_closes_descendant_pipes() {
        let mut unrelated=OwnedProcess::start(spec("sleep",128)).unwrap();
        let mut tree=OwnedProcess::start(spec("tree",4096)).unwrap();
        let start=Instant::now();
        loop {
            let(out,_)=tree.output().unwrap();
            let text=String::from_utf8_lossy(&out.bytes);
            if text.contains("VIVIM_ROOT_EXITING")&&text.contains("VIVIM_DESCENDANT_STARTED"){break;}
            assert!(start.elapsed()<Duration::from_secs(5),"descendant did not start");
            thread::sleep(Duration::from_millis(10));
        }
        assert!(tree.wait_for(Duration::from_millis(200)).unwrap().is_none());
        tree.cancel().unwrap();closed_output(&tree);
        assert!(unrelated.poll().unwrap().is_none(),"unrelated job was affected");
        unrelated.cancel().unwrap();
    }
    #[cfg(windows)]
    #[test]
    fn dropping_owned_job_closes_descendant_pipes() {
        let tree=OwnedProcess::start(spec("tree",4096)).unwrap();
        let start=Instant::now();
        loop {
            let(out,_)=tree.output().unwrap();
            if String::from_utf8_lossy(&out.bytes).contains("VIVIM_DESCENDANT_STARTED"){break;}
            assert!(start.elapsed()<Duration::from_secs(5),"descendant did not start");
            thread::sleep(Duration::from_millis(10));
        }
        let stdout=Arc::clone(&tree.stdout);let stderr=Arc::clone(&tree.stderr);
        drop(tree);
        let start=Instant::now();
        loop {
            if stdout.lock().unwrap().closed&&stderr.lock().unwrap().closed{break;}
            assert!(start.elapsed()<Duration::from_secs(5),"job drop did not close descendant pipes");
            thread::sleep(Duration::from_millis(10));
        }
    }
    #[test]
    fn rejects_relative_executable_and_invalid_capacity() {
        let mut s=spec("emit",128);s.executable=PathBuf::from("relative");
        assert!(matches!(OwnedProcess::start(s),Err(e) if e.kind()==io::ErrorKind::InvalidInput));
        assert!(matches!(OwnedProcess::start(spec("emit",MAX_TAIL+1)),Err(e) if e.kind()==io::ErrorKind::InvalidInput));
    }
}
