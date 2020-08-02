import os
import subprocess
import tempfile

types = ["u8", "u16", "u32", "u64", "s8", "s16", "s32", "s64", "f16", "f32", "f64"]
rnd = ["", ".rn", ".rni"]
ftz_all = ["", ".ftz"]
sat = ["", ".sat"]

for in_type in types:
    for out_type in types:
        for r in rnd:
            for ftz in ftz_all:
                for s in sat:
                    with tempfile.TemporaryDirectory() as dir:
                        f_name = os.path.join(dir, 'ptx')
                        out_name = os.path.join(dir, 'out')
                        with open(f_name, 'w') as f:
                            f.write(
                            f"""
                            .version 6.5
                            .target sm_30
                            .address_size 64
                            .visible .entry VecAdd_kernel()
                            {{
                                .reg.{in_type}              r1;
                                .reg.{out_type}             r2;
                                cvt{r}{ftz}{s}.{out_type}.{in_type}     r2, r1;
                                ret;
                            }}
                            """)
                        err = subprocess.run(f"ptxas {f_name} -o {out_name}", capture_output = True)
                        if err.returncode == 0:
                            print(f"cvt{r}{ftz}{s}.{out_type}.{in_type}")
                        #else:
                        #    print(f"[INVALID] cvt{r}{ftz}{s}.{out_type}.{in_type}")