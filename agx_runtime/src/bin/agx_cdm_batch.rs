//! `agx_cdm_batch MODULE.o MODULE.asm [--n THREADS] [--dispatches N]`: several dispatches in one compute command.
//! MODULE (zoc's output) has `inc(u32 *p, u32 n)` (`p[i] += 1`) and `twice(const u32 *src, u32 *dst, u32 n)`
//! (`dst[i] = 2 * src[i]`). Runs N `inc` launches as N commands and as one command whose control stream holds the N
//! dispatches, each followed by the barrier `cdm_dispatch_with` puts after a launch; checks that every word ends at
//! N (the dispatches ran in order and saw each other's stores), then `inc` and `twice` in one command (`twice` reads
//! what `inc` wrote). Prints one JSON line: the results and the host and firmware time of each form.
use agx_runtime::{module, Device, LoadedModule, Queue, Vm};
use std::time::Instant;

fn die(m: &str) -> ! {
    eprintln!("{{\"error\":{m:?}}}");
    std::process::exit(1)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 3 {
        die("usage: agx_cdm_batch MODULE.o MODULE.asm [--n THREADS] [--dispatches N]");
    }
    let (mut n, mut dispatches) = (65536u32, 64u32);
    let mut i = 3;
    while i + 1 < a.len() {
        match a[i].as_str() {
            "--n" => n = a[i + 1].parse().unwrap_or_else(|_| die("--n")),
            "--dispatches" => dispatches = a[i + 1].parse().unwrap_or_else(|_| die("--dispatches")),
            o => die(&format!("unknown option {o}")),
        }
        i += 2;
    }
    let object = std::fs::read(&a[1]).unwrap_or_else(|e| die(&e.to_string()));
    let asm = std::fs::read_to_string(&a[2]).unwrap_or_else(|e| die(&e.to_string()));
    let dev = Device::open("/dev/dri/renderD128").unwrap_or_else(|e| die(&e.to_string()));
    let g14x = dev.g14x();
    let freq = dev.params.command_timestamp_frequency_hz.max(1) as f64;
    let mut vm = Vm::new(&dev).unwrap_or_else(|e| die(&e.to_string()));
    let m = module::Module::load(&object, &asm).unwrap_or_else(|e| die(&format!("{e:?}")));
    let lm = LoadedModule::new(&mut vm, m).unwrap_or_else(|e| die(&e.to_string()));
    let mut queue = Queue::new(&mut vm).unwrap_or_else(|e| die(&e.to_string()));
    let bytes = n as u64 * 4;
    let mut buf = vm.alloc_data(bytes).unwrap_or_else(|e| die(&e.to_string()));
    let mut out = vm.alloc_data(bytes).unwrap_or_else(|e| die(&e.to_string()));
    let grid = [n.div_ceil(256), 1, 1];
    let local = [256, 1, 1];
    let p = buf.va.to_le_bytes();
    let o = out.va.to_le_bytes();
    let nb = n.to_le_bytes();
    let inc: [&[u8]; 2] = [&p, &nb];
    let twice: [&[u8]; 3] = [&p, &o, &nb];
    let count = |buf: &agx_runtime::Bo, want: u32| -> usize {
        buf.as_slice()[..bytes as usize].chunks(4).filter(|w| u32::from_le_bytes([w[0], w[1], w[2], w[3]]) != want).count()
    };
    let fw = |q: &Queue, first: u64, last: u64| -> f64 {
        // firmware time from the first command's start to the last one's end, in us
        let s = q.timing(agx_runtime::Fence { syncobj: q.last().syncobj, point: first }).map(|t| t.0);
        let e = q.timing(agx_runtime::Fence { syncobj: q.last().syncobj, point: last }).map(|t| t.1);
        match (s, e) {
            (Some(s), Some(e)) => e.saturating_sub(s) as f64 * 1e6 / freq,
            _ => -1.0,
        }
    };

    // N commands
    buf.as_mut_slice()[..bytes as usize].fill(0);
    let t = Instant::now();
    let first = queue.last().point + 1;
    for _ in 0..dispatches {
        lm.launch_params_async(&mut vm, &mut queue, g14x, "inc", &inc, grid, local, 0, &[], &[], &[])
            .unwrap_or_else(|e| die(&e.to_string()));
    }
    let host_separate = t.elapsed().as_secs_f64() * 1e6;
    queue.synchronize().unwrap_or_else(|e| die(&e.to_string()));
    let fw_separate = fw(&queue, first, queue.last().point);
    let wrong_separate = count(&buf, dispatches);

    // one command with N dispatches
    buf.as_mut_slice()[..bytes as usize].fill(0);
    let t = Instant::now();
    let mut words = Vec::new();
    let mut helper = (None, None);
    for _ in 0..dispatches {
        let (_, h, s) = lm
            .launch_params_words(&mut vm, &mut queue, g14x, "inc", &inc, grid, local, 0, &[], &mut words)
            .unwrap_or_else(|e| die(&e.to_string()));
        helper = (h, s);
    }
    let f = queue.submit_words(&words, helper.0, helper.1, &[]).unwrap_or_else(|e| die(&e.to_string()));
    let host_batched = t.elapsed().as_secs_f64() * 1e6;
    queue.synchronize().unwrap_or_else(|e| die(&e.to_string()));
    let fw_batched = fw(&queue, f.point, f.point);
    let wrong_batched = count(&buf, dispatches);

    // inc then twice in one command
    buf.as_mut_slice()[..bytes as usize].fill(5);
    out.as_mut_slice()[..bytes as usize].fill(0);
    let mut words = Vec::new();
    lm.launch_params_words(&mut vm, &mut queue, g14x, "inc", &inc, grid, local, 0, &[], &mut words)
        .unwrap_or_else(|e| die(&e.to_string()));
    lm.launch_params_words(&mut vm, &mut queue, g14x, "twice", &twice, grid, local, 0, &[], &mut words)
        .unwrap_or_else(|e| die(&e.to_string()));
    queue.submit_words(&words, None, None, &[]).unwrap_or_else(|e| die(&e.to_string()));
    queue.synchronize().unwrap_or_else(|e| die(&e.to_string()));
    let wrong_chain = count(&out, 2 * (0x0505_0505 + 1));
    if wrong_chain > 0 {
        let w = |b: &agx_runtime::Bo, i: usize| u32::from_le_bytes(b.as_slice()[4 * i..4 * i + 4].try_into().unwrap());
        eprintln!("chain: buf {:?} out {:?}", (0..4).map(|i| w(&buf, i)).collect::<Vec<_>>(), (0..4).map(|i| w(&out, i)).collect::<Vec<_>>());
    }

    println!(
        "{{\"threads\":{n},\"dispatches\":{dispatches},\"wrong_separate\":{wrong_separate},\"wrong_batched\":{wrong_batched},\
\"wrong_chain\":{wrong_chain},\"host_us_separate\":{host_separate:.1},\"host_us_batched\":{host_batched:.1},\
\"fw_us_separate\":{fw_separate:.1},\"fw_us_batched\":{fw_batched:.1}}}"
    );
}
