//! Host side of device `printf` (`vprintf`): decodes the records the AGX runtime library appends to
//! `__zluda_agx_printf_buffer` and formats them with C's `printf` rules.
//!
//! Buffer layout (ZLUDA `docs/src/agx.md`): a `u32` count of record bytes written (including dropped ones) at byte
//! 0, records from byte 8. A record is `u32 size` (multiple of 8), `u32 argsize`, `u64 format` (device address),
//! then the arguments, each aligned to its size: 4 bytes for `%c %d %i %o %u %x %X` and `*`, 8 bytes for their
//! `l`/`ll`/`z`/`j`/`t` forms, `%p`, `%s` and the floating point conversions (doubles, as C promotes them).

use std::ffi::CString;

pub const BUFFER_SYMBOL: &str = "__zluda_agx_printf_buffer";
/// Set to 1 by `__assertfail` (and by nothing else) before it traps.
pub const TRAP_FLAG_SYMBOL: &str = "__zluda_agx_trap_flag";
const HEADER: usize = 8;

/// The formatted output of a buffer, and how many record bytes did not fit.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Drained {
    pub text: Vec<u8>,
    pub dropped_bytes: usize,
}

/// Decodes every record. `read_str` returns the NUL-terminated string at a device address (format strings and
/// `%s` arguments), or `None` if the address is not readable from the host.
pub fn drain(buf: &[u8], read_str: impl Fn(u64) -> Option<Vec<u8>>) -> Drained {
    let mut out = Drained::default();
    if buf.len() < HEADER {
        return out;
    }
    let written = u32::from_le_bytes(buf[0..4].try_into().unwrap()) as usize;
    let end = (HEADER + written).min(buf.len());
    out.dropped_bytes = HEADER + written - end;
    let mut at = HEADER;
    while at + 16 <= end {
        let size = u32::from_le_bytes(buf[at..at + 4].try_into().unwrap()) as usize;
        let argsize = u32::from_le_bytes(buf[at + 4..at + 8].try_into().unwrap()) as usize;
        let format = u64::from_le_bytes(buf[at + 8..at + 16].try_into().unwrap());
        if size < 16 || size % 8 != 0 || at + size > end || 16 + argsize > size {
            // A record that was reserved but not written (or a corrupt one): stop here.
            break;
        }
        let args = &buf[at + 16..at + 16 + argsize];
        match read_str(format) {
            Some(f) => format_record(&f, args, &read_str, &mut out.text),
            None => out.text.extend_from_slice(format!("<printf: format at {format:#x} not readable>\n").as_bytes()),
        }
        at += size;
    }
    out
}

struct Args<'a> {
    data: &'a [u8],
    at: usize,
}

impl Args<'_> {
    fn take(&mut self, n: usize) -> u64 {
        self.at = self.at.div_ceil(n) * n;
        let mut v = [0u8; 8];
        if let Some(b) = self.data.get(self.at..self.at + n) {
            v[..n].copy_from_slice(b);
        }
        self.at += n;
        u64::from_le_bytes(v)
    }
}

fn format_record(format: &[u8], data: &[u8], read_str: &impl Fn(u64) -> Option<Vec<u8>>, out: &mut Vec<u8>) {
    let mut args = Args { data, at: 0 };
    let mut i = 0;
    while i < format.len() {
        if format[i] != b'%' {
            out.push(format[i]);
            i += 1;
            continue;
        }
        let start = i;
        i += 1;
        if format.get(i) == Some(&b'%') {
            out.push(b'%');
            i += 1;
            continue;
        }
        // flags, width, precision
        let mut stars = Vec::new();
        while i < format.len() && b"-+ #0".contains(&format[i]) {
            i += 1;
        }
        let digits_or_star = |i: &mut usize, stars: &mut Vec<i32>, args: &mut Args| {
            if format.get(*i) == Some(&b'*') {
                stars.push(args.take(4) as u32 as i32);
                *i += 1;
            } else {
                while *i < format.len() && format[*i].is_ascii_digit() {
                    *i += 1;
                }
            }
        };
        digits_or_star(&mut i, &mut stars, &mut args);
        if format.get(i) == Some(&b'.') {
            i += 1;
            digits_or_star(&mut i, &mut stars, &mut args);
        }
        let len_start = i;
        while i < format.len() && b"hlLqjzt".contains(&format[i]) {
            i += 1;
        }
        let long = format[len_start..i].iter().any(|c| b"lLqjzt".contains(c));
        let Some(&conv) = format.get(i) else {
            out.extend_from_slice(&format[start..]);
            break;
        };
        i += 1;
        // The C format for this one conversion, with the length modifier normalised to what snprintf receives.
        let mut spec: Vec<u8> = format[start..len_start].to_vec();
        let arg = match conv {
            b'd' | b'i' | b'o' | b'u' | b'x' | b'X' | b'c' => {
                let v = if long { args.take(8) } else { args.take(4) };
                if conv != b'c' {
                    spec.extend_from_slice(b"ll");
                }
                let v = match (conv, long) {
                    (b'd' | b'i', false) => v as u32 as i32 as i64 as u64,
                    (_, false) => v as u32 as u64,
                    _ => v,
                };
                Some(C::Int(v))
            }
            b'p' => Some(C::Int(args.take(8))).map(|v| {
                spec.extend_from_slice(b"ll");
                v
            }),
            b'f' | b'F' | b'e' | b'E' | b'g' | b'G' | b'a' | b'A' => Some(C::Double(f64::from_bits(args.take(8)))),
            b's' => {
                let p = args.take(8);
                Some(C::Str(if p == 0 { b"(null)".to_vec() } else { read_str(p).unwrap_or_else(|| b"<bad string>".to_vec()) }))
            }
            _ => None,
        };
        let Some(arg) = arg else {
            out.extend_from_slice(&format[start..i]);
            continue;
        };
        if conv == b'p' {
            // %p has no ll form: print it as C does on 64-bit Linux.
            spec.truncate(spec.len() - 2);
            spec.extend_from_slice(b"#llx");
            if let C::Int(0) = arg {
                spec = b"(nil)".to_vec();
            }
        } else {
            spec.push(conv);
        }
        out.extend_from_slice(&snprintf(&spec, &stars, &arg));
    }
}

enum C {
    Int(u64),
    Double(f64),
    Str(Vec<u8>),
}

fn snprintf(spec: &[u8], stars: &[i32], arg: &C) -> Vec<u8> {
    let Ok(f) = CString::new(spec) else { return spec.to_vec() };
    let s = match arg {
        C::Str(b) => CString::new(b.iter().copied().take_while(|&c| c != 0).collect::<Vec<u8>>()).unwrap(),
        _ => CString::default(),
    };
    let call = |buf: *mut libc::c_char, n: usize| unsafe {
        let f = f.as_ptr();
        match (arg, stars) {
            (C::Int(v), []) => libc::snprintf(buf, n, f, *v),
            (C::Int(v), [a]) => libc::snprintf(buf, n, f, *a, *v),
            (C::Int(v), [a, b, ..]) => libc::snprintf(buf, n, f, *a, *b, *v),
            (C::Double(v), []) => libc::snprintf(buf, n, f, *v),
            (C::Double(v), [a]) => libc::snprintf(buf, n, f, *a, *v),
            (C::Double(v), [a, b, ..]) => libc::snprintf(buf, n, f, *a, *b, *v),
            (C::Str(_), []) => libc::snprintf(buf, n, f, s.as_ptr()),
            (C::Str(_), [a]) => libc::snprintf(buf, n, f, *a, s.as_ptr()),
            (C::Str(_), [a, b, ..]) => libc::snprintf(buf, n, f, *a, *b, s.as_ptr()),
        }
    };
    let n = call(std::ptr::null_mut(), 0);
    if n < 0 {
        return spec.to_vec();
    }
    let mut buf = vec![0u8; n as usize + 1];
    call(buf.as_mut_ptr() as *mut libc::c_char, buf.len());
    buf.truncate(n as usize);
    buf
}

/// Clears the record count so the next launch starts an empty buffer.
pub fn reset(buf: &mut [u8]) {
    buf[..4].fill(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Rec(Vec<u8>);
    impl Rec {
        fn new() -> Rec {
            Rec(Vec::new())
        }
        fn arg(mut self, bytes: &[u8]) -> Rec {
            let n = bytes.len();
            while self.0.len() % n != 0 {
                self.0.push(0);
            }
            self.0.extend_from_slice(bytes);
            self
        }
    }

    fn buffer(records: &[(u64, Rec)]) -> Vec<u8> {
        let mut b = vec![0u8; 8];
        for (fmt, r) in records {
            let size = (16 + r.0.len()).div_ceil(8) * 8;
            b.extend_from_slice(&(size as u32).to_le_bytes());
            b.extend_from_slice(&(r.0.len() as u32).to_le_bytes());
            b.extend_from_slice(&fmt.to_le_bytes());
            b.extend_from_slice(&r.0);
            b.resize(b.len().div_ceil(8) * 8, 0);
        }
        let n = (b.len() - 8) as u32;
        b[..4].copy_from_slice(&n.to_le_bytes());
        b
    }

    fn strings(addr: u64) -> Option<Vec<u8>> {
        match addr {
            0x100 => Some(b"x=%d u=%u h=%#06x c=%c s=%s\n".to_vec()),
            0x200 => Some(b"%lld %llx %5.2f %e %g %p %p %% %*d|%-*.*f|\n".to_vec()),
            0x300 => Some(b"world".to_vec()),
            _ => None,
        }
    }

    #[test]
    fn formats_like_c() {
        let r1 = Rec::new()
            .arg(&(-7i32).to_le_bytes())
            .arg(&u32::MAX.to_le_bytes())
            .arg(&0xabu32.to_le_bytes())
            .arg(&(b'Z' as u32).to_le_bytes())
            .arg(&0x300u64.to_le_bytes());
        let r2 = Rec::new()
            .arg(&(-1i64).to_le_bytes())
            .arg(&0xdead_beef_cafeu64.to_le_bytes())
            .arg(&3.14159f64.to_le_bytes())
            .arg(&1e-10f64.to_le_bytes())
            .arg(&0.5f64.to_le_bytes())
            .arg(&0x1234u64.to_le_bytes())
            .arg(&0u64.to_le_bytes())
            .arg(&4u32.to_le_bytes())
            .arg(&42u32.to_le_bytes())
            .arg(&6u32.to_le_bytes())
            .arg(&1u32.to_le_bytes())
            .arg(&2.25f64.to_le_bytes());
        let d = drain(&buffer(&[(0x100, r1), (0x200, r2)]), strings);
        assert_eq!(
            String::from_utf8(d.text).unwrap(),
            "x=-7 u=4294967295 h=0x00ab c=Z s=world\n-1 deadbeefcafe  3.14 1.000000e-10 0.5 0x1234 (nil) %   42|2.2   |\n"
        );
        assert_eq!(d.dropped_bytes, 0);
    }

    // The record __assertfail wrote for thread 1 of `assert(x > 0)` in agx_run (globals at 0x4000000: the
    // message, file and function strings at 0, 6 and 11, the format at 0x10018).
    #[test]
    fn formats_assert_records_from_the_simulator() {
        let rec = "060000040000000007000000000000000b000004000000000000000000000000\
                   000000000100000000000000000000000000000400000000";
        let args: Vec<u8> = (0..rec.len()).step_by(2).map(|i| u8::from_str_radix(&rec[i..i + 2], 16).unwrap()).collect();
        let mut b = vec![0u8; 8];
        b.extend_from_slice(&72u32.to_le_bytes());
        b.extend_from_slice(&56u32.to_le_bytes());
        b.extend_from_slice(&0x4010018u64.to_le_bytes());
        b.extend_from_slice(&args);
        b[..4].copy_from_slice(&72u32.to_le_bytes());
        let read = |a: u64| match a {
            0x4000000 => Some(b"x > 0".to_vec()),
            0x4000006 => Some(b"a.cu".to_vec()),
            0x400000b => Some(b"k".to_vec()),
            0x4010018 => Some(b"%s:%u: %s: block: [%u,%u,%u], thread: [%u,%u,%u] Assertion `%s` failed.\n".to_vec()),
            _ => None,
        };
        assert_eq!(
            String::from_utf8(drain(&b, read).text).unwrap(),
            "a.cu:7: k: block: [0,0,0], thread: [1,0,0] Assertion `x > 0` failed.\n"
        );
    }

    #[test]
    fn reports_dropped_and_unreadable_records() {
        let mut b = buffer(&[(0x999, Rec::new())]);
        // The device counted 64 more bytes than the buffer holds.
        let n = u32::from_le_bytes(b[..4].try_into().unwrap()) + 64;
        b[..4].copy_from_slice(&n.to_le_bytes());
        let d = drain(&b, strings);
        assert_eq!(d.dropped_bytes, 64);
        assert_eq!(String::from_utf8(d.text).unwrap(), "<printf: format at 0x999 not readable>\n");
        reset(&mut b);
        assert_eq!(drain(&b, strings), Drained::default());
    }
}
