use crate::cuda_call;
use crate::try_get_cuda_function;
use crate::CudaDynamicFns;
use crate::{
    log::{FunctionLogger, LogEntry},
    Settings,
};
use crossbeam_channel::{Receiver, Sender};
use cuda_types::*;
use rustc_hash::FxHashSet;
use serde::Serialize;
use std::borrow::Cow;
use std::env;
use std::ffi::CStr;
use std::fmt::Display;
use std::io::BufWriter;
use std::io::Write;
use std::process;
use std::sync::Arc;
use std::time::Instant;
use std::{
    collections::VecDeque,
    fs::File,
    path::PathBuf,
    ptr,
    thread::{self, JoinHandle},
    time::Duration,
};

pub(crate) struct Profiler {
    cu_event_destroy_v2: extern "system" fn(CUevent) -> CUresult,
    sender: Sender<ProfilerPacket>,
    _thread: JoinHandle<()>,
}

impl Profiler {
    pub(crate) fn new(
        settings: &Settings,
        fn_table: &mut CudaDynamicFns,
        logger: &mut FunctionLogger,
    ) -> Option<Self> {
        logger
            .log_unwrap(Self::new_impl(settings, fn_table))
            .flatten()
    }

    pub(crate) fn record_kernel(
        &self,
        stream: CUstream,
        function: FunctionName,
        start: CUevent,
        end: CUevent,
    ) {
        self.sender
            .send(ProfilerPacket::RecordKernel(KernelEnqueue {
                stream,
                function,
                start,
                end,
                cu_event_destroy_v2: self.cu_event_destroy_v2,
            }))
            .ok();
    }

    pub(crate) fn record_task(&self, function: &'static str) -> TimedTask {
        let thread_id = thread_id::get();
        TimedTask {
            profiler: self,
            name: function,
            start: Instant::now(),
            thread_id,
        }
    }

    fn new_impl(
        settings: &Settings,
        fn_table: &mut CudaDynamicFns,
    ) -> Result<Option<Self>, LogEntry> {
        let path = match &settings.profiler_output {
            Some(path) => {
                let path = PathBuf::from(path);
                if path.is_absolute() {
                    path
                } else {
                    let mut base_dir = match &settings.dump_dir {
                        Some(dump_dir) => dump_dir.clone(),
                        None => std::env::current_dir()?,
                    };
                    base_dir.push(path);
                    base_dir
                }
            }
            None => return Ok(None),
        };
        let file = File::create(path)?;
        let cu_event_create = try_get_cuda_function!(fn_table, cuEventCreate)?;
        let cu_event_destroy_v2 = try_get_cuda_function!(fn_table, cuEventDestroy_v2)?;
        let cu_event_query = try_get_cuda_function!(fn_table, cuEventQuery)?;
        let cu_event_elapsed_time = try_get_cuda_function!(fn_table, cuEventElapsedTime)?;
        let mut profiling_start = ptr::null_mut();
        cuda_call!(fn_table.cuInit(0));
        let device_name = Self::get_device_name(fn_table)?;
        let mut ctx = ptr::null_mut();
        cuda_call!(fn_table.cuDevicePrimaryCtxRetain(&mut ctx, CUdevice_v1(0)));
        cuda_call!(fn_table.cuCtxPushCurrent_v2(ctx));
        cuda_call!(cu_event_create(&mut profiling_start, 0));
        cuda_call!(fn_table.cuEventRecord(profiling_start, ptr::null_mut()));
        let host_start = Instant::now();
        cuda_call!(fn_table.cuEventSynchronize(profiling_start));
        cuda_call!(fn_table.cuCtxPopCurrent_v2(&mut ctx));
        // Don't release the primary context, otherwise the event will get wiped out
        let (sender, receiver) = crossbeam_channel::bounded(1);
        let kernel_start = ForceSend(profiling_start);
        let thread = thread::spawn(move || {
            Self::run(
                file,
                device_name,
                cu_event_query,
                cu_event_elapsed_time,
                host_start,
                kernel_start,
                receiver,
            )
        });
        Ok(Some(Self {
            cu_event_destroy_v2,
            sender,
            _thread: thread,
        }))
    }

    fn get_device_name(fn_table: &mut CudaDynamicFns) -> Result<String, LogEntry> {
        let mut name = vec![0i8; 256];
        cuda_call!(fn_table.cuDeviceGetName(name.as_mut_ptr(), name.len() as i32, CUdevice_v1(0)));
        Ok(unsafe { CStr::from_ptr(name.as_ptr()) }
            .to_str()
            .unwrap()
            .to_string())
    }

    fn run(
        file: File,
        device_name: String,
        cu_event_query: extern "system" fn(CUevent) -> CUresult,
        cu_event_elapsed_time: extern "system" fn(*mut f32, CUevent, CUevent) -> CUresult,
        host_start: Instant,
        kernel_start: ForceSend<CUevent>,
        receiver: Receiver<ProfilerPacket>,
    ) {
        let mut writer = ProfilingWriter::new(Self::current_exe(), file, device_name);
        let profiling_start = kernel_start.0;
        let mut queue = VecDeque::new();
        let mut timeout = 1;
        'thread_loop: loop {
            match receiver.recv_timeout(Duration::from_millis(timeout)) {
                Ok(ProfilerPacket::Finish) => return,
                Ok(ProfilerPacket::RecordKernel(packet)) => {
                    timeout = 1;
                    queue.push_back(packet);
                    loop {
                        match receiver.try_recv() {
                            Ok(ProfilerPacket::Finish) => return,
                            Ok(ProfilerPacket::RecordTask(task)) => {
                                Self::process_task(&mut writer, host_start, task)
                            }
                            Ok(ProfilerPacket::RecordKernel(packet)) => queue.push_back(packet),
                            Err(_) => continue 'thread_loop,
                        }
                    }
                }
                Ok(ProfilerPacket::RecordTask(task)) => {
                    timeout = 1;
                    Self::process_task(&mut writer, host_start, task)
                }
                Err(_) => {
                    timeout = u64::max(100, timeout * 2);
                    Self::process_queue(
                        &mut writer,
                        profiling_start,
                        cu_event_query,
                        cu_event_elapsed_time,
                        &mut queue,
                    );
                }
            }
        }
    }

    fn current_exe() -> Option<String> {
        env::current_exe()
            .map(|path| {
                path.file_name()
                    .map(|exe| exe.to_string_lossy().to_string())
            })
            .unwrap_or(None)
    }

    fn process_task(writer: &mut ProfilingWriter, host_start: Instant, task: TaskMeasurement) {
        let time_from_start = task.start.saturating_duration_since(host_start);
        let duration = task.end.saturating_duration_since(task.start);
        writer.write_host(task.thread_id, task.function, time_from_start, duration)
    }

    fn process_queue(
        writer: &mut ProfilingWriter,
        first_event: CUevent,
        cu_event_query: extern "system" fn(CUevent) -> CUresult,
        cu_event_elapsed_time: extern "system" fn(*mut f32, CUevent, CUevent) -> CUresult,
        queue: &mut VecDeque<KernelEnqueue>,
    ) {
        loop {
            let should_dequeue = match queue.front() {
                Some(packet) => cu_event_query(packet.end) == CUresult::CUDA_SUCCESS,
                None => false,
            };
            if !should_dequeue {
                return;
            }
            let kernel = queue.pop_front().unwrap();
            let mut time_from_start = 0f32;
            let cu_result = cu_event_elapsed_time(&mut time_from_start, first_event, kernel.start);
            if cu_result != CUresult::CUDA_SUCCESS {
                panic!("{}", cu_result.0);
            }
            let mut time_of_execution = 0f32;
            let cu_result = cu_event_elapsed_time(&mut time_of_execution, kernel.start, kernel.end);
            if cu_result != CUresult::CUDA_SUCCESS {
                panic!("{}", cu_result.0);
            }
            writer.write_kernel(
                kernel.stream,
                &kernel.function,
                time_from_start,
                time_of_execution,
            );
        }
    }
}

impl Drop for Profiler {
    fn drop(&mut self) {
        self.sender.send(ProfilerPacket::Finish).ok();
    }
}

pub(crate) struct TimedTask<'a> {
    profiler: &'a Profiler,
    name: &'static str,
    start: Instant,
    thread_id: usize,
}

impl<'a> Drop for TimedTask<'a> {
    fn drop(&mut self) {
        let end = Instant::now();
        self.profiler
            .sender
            .send(ProfilerPacket::RecordTask(TaskMeasurement {
                function: self.name,
                start: self.start,
                end,
                thread_id: self.thread_id,
            }))
            .ok();
    }
}

enum ProfilerPacket {
    RecordKernel(KernelEnqueue),
    RecordTask(TaskMeasurement),
    Finish,
}

struct KernelEnqueue {
    stream: CUstream,
    function: FunctionName,
    start: CUevent,
    end: CUevent,
    cu_event_destroy_v2: extern "system" fn(CUevent) -> CUresult,
}

#[allow(unused_must_use)]
impl Drop for KernelEnqueue {
    fn drop(&mut self) {
        (self.cu_event_destroy_v2)(self.start);
        (self.cu_event_destroy_v2)(self.end);
    }
}

struct TaskMeasurement {
    function: &'static str,
    start: Instant,
    end: Instant,
    thread_id: usize,
}

unsafe impl Send for KernelEnqueue {}

struct ForceSend<T>(T);

unsafe impl<T> Send for ForceSend<T> {}

pub(crate) enum FunctionName {
    Resolved(Arc<String>),
    Unresolved(CUfunction),
}

impl Display for FunctionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionName::Resolved(name) => Display::fmt(name, f),
            FunctionName::Unresolved(cu_func) => write!(f, "{:p}", cu_func),
        }
    }
}

struct ProfilingWriter {
    file: BufWriter<File>,
    known_streams: FxHashSet<CUstream>,
    pid: u32,
}

impl ProfilingWriter {
    fn new(process: Option<String>, mut file: File, device_name: String) -> Self {
        let pid = process::id();
        file.write_all(b"[\n").unwrap();
        let mut serializer = serde_json::Serializer::new(&mut file);
        let entry = ProfilingEvent::Metadata {
            name: "process_name",
            ph: "M",
            pid: 0,
            tid: 0,
            args: NameArg { name: device_name },
        };
        entry.serialize(&mut serializer).unwrap();
        drop(serializer);
        if let Some(exe) = process {
            file.write_all(b",\n").unwrap();
            let mut serializer = serde_json::Serializer::new(&mut file);
            let entry = ProfilingEvent::Metadata {
                name: "process_name",
                ph: "M",
                pid,
                tid: 0,
                args: NameArg { name: exe },
            };
            entry.serialize(&mut serializer).unwrap();
        }
        let file = BufWriter::new(file);
        Self {
            file,
            known_streams: FxHashSet::default(),
            pid,
        }
    }

    fn write_host(
        &mut self,
        tid: usize,
        func: &'static str,
        time_from_start: Duration,
        duration: Duration,
    ) {
        self.file.write_all(b",\n").unwrap();
        let mut serializer = serde_json::Serializer::new(&mut self.file);
        let entry = ProfilingEvent::Complete {
            name: Cow::Borrowed(func),
            cat: "host",
            ph: "X",
            ts: time_from_start.as_micros() as f32,
            dur: duration.as_micros() as f32,
            pid: self.pid,
            tid,
        };
        entry.serialize(&mut serializer).unwrap();
    }

    fn write_kernel(
        &mut self,
        stream: CUstream,
        function: &FunctionName,
        millis_from_start: f32,
        millis_duration: f32,
    ) {
        if self.known_streams.insert(stream) {
            self.file.write_all(b",\n").unwrap();
            let mut serializer = serde_json::Serializer::new(&mut self.file);
            let entry = ProfilingEvent::Metadata {
                name: "thread_name",
                ph: "M",
                pid: 0,
                tid: stream as usize,
                args: NameArg {
                    name: format!("Stream {:p}", stream),
                },
            };
            entry.serialize(&mut serializer).unwrap();
        }
        self.file.write_all(b",\n").unwrap();
        let mut serializer = serde_json::Serializer::new(&mut self.file);
        let ts = millis_from_start * 1000f32;
        let dur = millis_duration * 1000f32;
        let entry = ProfilingEvent::Complete {
            name: Cow::Owned(format!("{}", function)),
            cat: "kernel",
            ph: "X",
            ts: ts,
            dur: dur,
            pid: 0,
            tid: stream as usize,
        };
        entry.serialize(&mut serializer).unwrap();
    }
}

impl Drop for ProfilingWriter {
    fn drop(&mut self) {
        self.file.write_all(b"\n]").ok();
    }
}

// https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU/preview
#[derive(Serialize)]
#[serde(untagged)]
enum ProfilingEvent {
    Complete {
        name: Cow<'static, str>,
        cat: &'static str,
        ph: &'static str,
        ts: f32,
        dur: f32,
        pid: u32,
        tid: usize,
    },
    Metadata {
        name: &'static str,
        ph: &'static str,
        pid: u32,
        tid: usize,
        args: NameArg,
    },
}

#[derive(Serialize)]
struct NameArg {
    name: String,
}
