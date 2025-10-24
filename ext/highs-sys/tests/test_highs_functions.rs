use std::convert::TryInto;
use std::ffi::CString;

use highs_sys::*;

fn c(n: usize) -> HighsInt {
    n.try_into().unwrap()
}

fn ptr<T>(a: &mut [T]) -> *mut T {
    a.as_mut_ptr()
}

#[test]
fn highs_functions() {
    unsafe {
        // Form and solve the LP
        // Max    f  = 2x_0 + 3x_1
        // s.t.                x_1 <= 6
        //       10 <=  x_0 + 2x_1 <= 14
        //        8 <= 2x_0 +  x_1
        // 0 <= x_0 <= 3; 1 <= x_1

        let highs = Highs_create();

        // Solving the problem without printing to the standard output
        let option_name = CString::new("output_flag").unwrap();
        Highs_setBoolOptionValue(highs, option_name.as_ptr(), 0);

        let numcol: usize = 2;
        let numrow: usize = 3;
        let nnz: usize = 5;

        let inf = Highs_getInfinity(highs);

        // Define the column costs, lower bounds and upper bounds
        let colcost: &mut [f64] = &mut [2.0, 3.0];
        let collower: &mut [f64] = &mut [0.0, 1.0];
        let colupper: &mut [f64] = &mut [3.0, inf];
        // Define the row lower bounds and upper bounds
        let rowlower: &mut [f64] = &mut [-inf, 10.0, 8.0];
        let rowupper: &mut [f64] = &mut [6.0, 14.0, inf];

        // Define the constraint matrix row-wise, as it is added to the LP
        // with the rows
        let arstart: &mut [HighsInt] = &mut [0, 1, 3];
        let arindex: &mut [HighsInt] = &mut [1, 0, 1, 0, 1];
        let arvalue: &mut [f64] = &mut [1.0, 1.0, 2.0, 2.0, 1.0];

        use std::ptr::null;

        // Add two columns to the empty LP
        let success = Highs_addCols(
            highs,
            c(numcol),
            ptr(colcost),
            ptr(collower),
            ptr(colupper),
            0,
            null(),
            null(),
            null(),
        );
        assert_eq!(STATUS_OK, success, "addCols");
        // Add three rows to the 2-column LP
        let success = Highs_addRows(
            highs,
            c(numrow),
            ptr(rowlower),
            ptr(rowupper),
            c(nnz),
            ptr(arstart),
            ptr(arindex),
            ptr(arvalue),
        );
        assert_eq!(STATUS_OK, success, "addRows");

        let success = Highs_changeObjectiveSense(highs, OBJECTIVE_SENSE_MAXIMIZE);
        assert_eq!(success, STATUS_OK);

        let simplex_scale_strategy = 3;
        let option_name = CString::new("simplex_scale_strategy").unwrap();
        Highs_setIntOptionValue(highs, option_name.as_ptr(), simplex_scale_strategy);

        let status = Highs_run(highs);
        assert_eq!(status, STATUS_OK);

        let model_status = Highs_getModelStatus(highs);
        assert_eq!(model_status, MODEL_STATUS_OPTIMAL);

        let mut objective_function_value = 0.;
        let info_name = CString::new("objective_function_value").unwrap();
        Highs_getHighsDoubleInfoValue(
            highs,
            info_name.as_ptr(),
            (&mut objective_function_value) as *mut f64,
        );
        assert_eq!(objective_function_value, 2. * 3. + 3. * 5.5);

        let colvalue: &mut [f64] = &mut vec![0.; numcol];
        let coldual: &mut [f64] = &mut vec![0.; numcol];
        let rowvalue: &mut [f64] = &mut vec![0.; numrow];
        let rowdual: &mut [f64] = &mut vec![0.; numrow];

        // Get the primal and dual solution
        Highs_getSolution(
            highs,
            ptr(colvalue),
            ptr(coldual),
            ptr(rowvalue),
            ptr(rowdual),
        );
        assert_eq!(colvalue, &[3.0, 5.5]);

        let colbasisstatus: &mut [HighsInt] = &mut vec![0; numcol];
        let rowbasisstatus: &mut [HighsInt] = &mut vec![0; numrow];
        // Get the basis
        Highs_getBasis(highs, ptr(colbasisstatus), ptr(rowbasisstatus));
        assert_eq!(colbasisstatus, &[2, 1]);

        Highs_destroy(highs);
    }
}

#[cfg(not(target_os = "windows"))] // broken on windows
#[test]
fn highs_functions_multithread() {
    let threads: Vec<_> = (0..128)
        .map(|_| std::thread::spawn(highs_functions))
        .collect();
    for t in threads {
        t.join().expect("Thread should not panic");
    }
}
