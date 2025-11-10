use highs_sys::*;
use std::convert::TryInto;

#[test]
fn highs_call() {
    // This illustrates the use of Highs_call, the simple C interface to
    // HiGHS. It's designed to solve the general LP problem
    //
    // Min c^Tx subject to L <= Ax <= U; l <= x <= u
    //
    // where A is a matrix with m rows and n columns
    //
    // The scalar n is numcol
    // The scalar m is numrow
    //
    // The vector c is colcost
    // The vector l is collower
    // The vector u is colupper
    // The vector L is rowlower
    // The vector U is rowupper
    //
    // The matrix A is represented in packed column-wise form: only its
    // nonzeros are stored
    //
    // * The number of nonzeros in A is nnz
    //
    // * The row indices of the nonnzeros in A are stored column-by-column
    // in aindex
    //
    // * The values of the nonnzeros in A are stored column-by-column in
    // avalue
    //
    // * The position in aindex/avalue of the index/value of the first
    // nonzero in each column is stored in astart
    //
    // Note that astart[0] must be zero
    //
    // After a successful call to Highs_call, the primal and dual
    // solution, and the simplex basis are returned as follows
    //
    // The vector x is colvalue
    // The vector Ax is rowvalue
    // The vector of dual values for the variables x is coldual
    // The vector of dual values for the variables Ax is rowdual
    // The basic/nonbasic status of the variables x is colbasisstatus
    // The basic/nonbasic status of the variables Ax is rowbasisstatus
    //
    // The status of the solution obtained is modelstatus
    //
    // To solve maximization problems, the values in c must be negated
    //
    // The use of Highs_call is illustrated for the LP
    //
    // Min    f  = 2x_0 + 3x_1
    // s.t.                x_1 <= 6
    //       10 <=  x_0 + 2x_1 <= 14
    //        8 <= 2x_0 +  x_1
    // 0 <= x_0 <= 3; 1 <= x_1

    let numcol: usize = 2;
    let numrow: usize = 3;
    let nnz: usize = 5;

    // Define the column costs, lower bounds and upper bounds
    let colcost: &[f64] = &[2.0, 3.0];
    let collower: &[f64] = &[0.0, 1.0];
    let colupper: &[f64] = &[3.0, 1.0e30];
    // Define the row lower bounds and upper bounds
    let rowlower: &[f64] = &[-1.0e30, 10.0, 8.0];
    let rowupper: &[f64] = &[6.0, 14.0, 1.0e30];
    // Define the constraint matrix column-wise
    let astart: &[HighsInt] = &[0, 2];
    let aindex: &[HighsInt] = &[1, 2, 0, 1, 2];
    let avalue: &[f64] = &[1.0, 2.0, 1.0, 2.0, 1.0];

    let colvalue: &mut [f64] = &mut vec![0.; numcol];
    let coldual: &mut [f64] = &mut vec![0.; numcol];
    let rowvalue: &mut [f64] = &mut vec![0.; numrow];
    let rowdual: &mut [f64] = &mut vec![0.; numrow];

    let colbasisstatus: &mut [HighsInt] = &mut vec![0; numcol];
    let rowbasisstatus: &mut [HighsInt] = &mut vec![0; numrow];

    let modelstatus: &mut HighsInt = &mut 0;
    let offset = 0.0;

    let status: HighsInt = unsafe {
        Highs_lpCall(
            numcol.try_into().unwrap(),
            numrow.try_into().unwrap(),
            nnz.try_into().unwrap(),
            MATRIX_FORMAT_COLUMN_WISE,
            OBJECTIVE_SENSE_MINIMIZE,
            offset,
            colcost.as_ptr(),
            collower.as_ptr(),
            colupper.as_ptr(),
            rowlower.as_ptr(),
            rowupper.as_ptr(),
            astart.as_ptr(),
            aindex.as_ptr(),
            avalue.as_ptr(),
            colvalue.as_mut_ptr(),
            coldual.as_mut_ptr(),
            rowvalue.as_mut_ptr(),
            rowdual.as_mut_ptr(),
            colbasisstatus.as_mut_ptr(),
            rowbasisstatus.as_mut_ptr(),
            modelstatus,
        )
    };

    assert_eq!(status, STATUS_OK);
    assert_eq!(colvalue, &[2., 4.]);
}
