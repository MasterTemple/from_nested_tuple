use super::FromNestedTuple;

impl<T0, T1> FromNestedTuple for (T0, T1) {
    type Tuple = (T0, T1);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (t0, t1) = tuple;
        (t0, t1)
    }
}

impl<T0, T1, T2> FromNestedTuple for (T0, T1, T2) {
    type Tuple = ((T0, T1), T2);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((t0, t1), t2) = tuple;
        (t0, t1, t2)
    }
}

impl<T0, T1, T2, T3> FromNestedTuple for (T0, T1, T2, T3) {
    type Tuple = (((T0, T1), T2), T3);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((t0, t1), t2), t3) = tuple;
        (t0, t1, t2, t3)
    }
}

impl<T0, T1, T2, T3, T4> FromNestedTuple for (T0, T1, T2, T3, T4) {
    type Tuple = ((((T0, T1), T2), T3), T4);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((t0, t1), t2), t3), t4) = tuple;
        (t0, t1, t2, t3, t4)
    }
}

impl<T0, T1, T2, T3, T4, T5> FromNestedTuple for (T0, T1, T2, T3, T4, T5) {
    type Tuple = (((((T0, T1), T2), T3), T4), T5);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((t0, t1), t2), t3), t4), t5) = tuple;
        (t0, t1, t2, t3, t4, t5)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6) {
    type Tuple = ((((((T0, T1), T2), T3), T4), T5), T6);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((t0, t1), t2), t3), t4), t5), t6) = tuple;
        (t0, t1, t2, t3, t4, t5, t6)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Tuple = (((((((T0, T1), T2), T3), T4), T5), T6), T7);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((t0, t1), t2), t3), t4), t5), t6), t7) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Tuple = ((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> FromNestedTuple
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
{
    type Tuple = (((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> FromNestedTuple
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
    type Tuple = (
        (((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9),
        T10,
    );

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10) =
            tuple;
        (
            t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10,
        )
    }
}


impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Tuple = (((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11)
    }
}


impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Tuple = ((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Tuple = (((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Tuple = ((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Tuple = (((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type Tuple = ((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type Tuple = (((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type Tuple = ((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type Tuple = (((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type Tuple = ((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type Tuple = (((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type Tuple = ((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type Tuple = (((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type Tuple = ((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type Tuple = (((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type Tuple = ((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type Tuple = (((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type Tuple = ((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type Tuple = (((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type Tuple = ((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type Tuple = (((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32) {
    type Tuple = ((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33) {
    type Tuple = (((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34) {
    type Tuple = ((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35) {
    type Tuple = (((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36) {
    type Tuple = ((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37) {
    type Tuple = (((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38) {
    type Tuple = ((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39) {
    type Tuple = (((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40) {
    type Tuple = ((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41) {
    type Tuple = (((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42) {
    type Tuple = ((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43) {
    type Tuple = (((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44) {
    type Tuple = ((((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43), T44);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43), t44) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45) {
    type Tuple = (((((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43), T44), T45);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43), t44), t45) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46) {
    type Tuple = ((((((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43), T44), T45), T46);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43), t44), t45), t46) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47) {
    type Tuple = (((((((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43), T44), T45), T46), T47);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43), t44), t45), t46), t47) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46, t47)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48) {
    type Tuple = ((((((((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43), T44), T45), T46), T47), T48);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43), t44), t45), t46), t47), t48) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46, t47, t48)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49) {
    type Tuple = (((((((((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43), T44), T45), T46), T47), T48), T49);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43), t44), t45), t46), t47), t48), t49) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46, t47, t48, t49)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50) {
    type Tuple = ((((((((((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43), T44), T45), T46), T47), T48), T49), T50);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43), t44), t45), t46), t47), t48), t49), t50) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46, t47, t48, t49, t50)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51) {
    type Tuple = (((((((((((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43), T44), T45), T46), T47), T48), T49), T50), T51);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43), t44), t45), t46), t47), t48), t49), t50), t51) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46, t47, t48, t49, t50, t51)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52) {
    type Tuple = ((((((((((((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43), T44), T45), T46), T47), T48), T49), T50), T51), T52);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43), t44), t45), t46), t47), t48), t49), t50), t51), t52) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46, t47, t48, t49, t50, t51, t52)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53) {
    type Tuple = (((((((((((((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43), T44), T45), T46), T47), T48), T49), T50), T51), T52), T53);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43), t44), t45), t46), t47), t48), t49), t50), t51), t52), t53) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46, t47, t48, t49, t50, t51, t52, t53)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54) {
    type Tuple = ((((((((((((((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43), T44), T45), T46), T47), T48), T49), T50), T51), T52), T53), T54);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43), t44), t45), t46), t47), t48), t49), t50), t51), t52), t53), t54) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46, t47, t48, t49, t50, t51, t52, t53, t54)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55) {
    type Tuple = (((((((((((((((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43), T44), T45), T46), T47), T48), T49), T50), T51), T52), T53), T54), T55);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43), t44), t45), t46), t47), t48), t49), t50), t51), t52), t53), t54), t55) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46, t47, t48, t49, t50, t51, t52, t53, t54, t55)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56) {
    type Tuple = ((((((((((((((((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43), T44), T45), T46), T47), T48), T49), T50), T51), T52), T53), T54), T55), T56);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43), t44), t45), t46), t47), t48), t49), t50), t51), t52), t53), t54), t55), t56) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46, t47, t48, t49, t50, t51, t52, t53, t54, t55, t56)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57) {
    type Tuple = (((((((((((((((((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43), T44), T45), T46), T47), T48), T49), T50), T51), T52), T53), T54), T55), T56), T57);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43), t44), t45), t46), t47), t48), t49), t50), t51), t52), t53), t54), t55), t56), t57) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46, t47, t48, t49, t50, t51, t52, t53, t54, t55, t56, t57)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57, T58> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57, T58) {
    type Tuple = ((((((((((((((((((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43), T44), T45), T46), T47), T48), T49), T50), T51), T52), T53), T54), T55), T56), T57), T58);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43), t44), t45), t46), t47), t48), t49), t50), t51), t52), t53), t54), t55), t56), t57), t58) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46, t47, t48, t49, t50, t51, t52, t53, t54, t55, t56, t57, t58)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57, T58, T59> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57, T58, T59) {
    type Tuple = (((((((((((((((((((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43), T44), T45), T46), T47), T48), T49), T50), T51), T52), T53), T54), T55), T56), T57), T58), T59);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43), t44), t45), t46), t47), t48), t49), t50), t51), t52), t53), t54), t55), t56), t57), t58), t59) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46, t47, t48, t49, t50, t51, t52, t53, t54, t55, t56, t57, t58, t59)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57, T58, T59, T60> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57, T58, T59, T60) {
    type Tuple = ((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43), T44), T45), T46), T47), T48), T49), T50), T51), T52), T53), T54), T55), T56), T57), T58), T59), T60);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43), t44), t45), t46), t47), t48), t49), t50), t51), t52), t53), t54), t55), t56), t57), t58), t59), t60) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46, t47, t48, t49, t50, t51, t52, t53, t54, t55, t56, t57, t58, t59, t60)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57, T58, T59, T60, T61> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57, T58, T59, T60, T61) {
    type Tuple = (((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43), T44), T45), T46), T47), T48), T49), T50), T51), T52), T53), T54), T55), T56), T57), T58), T59), T60), T61);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43), t44), t45), t46), t47), t48), t49), t50), t51), t52), t53), t54), t55), t56), t57), t58), t59), t60), t61) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46, t47, t48, t49, t50, t51, t52, t53, t54, t55, t56, t57, t58, t59, t60, t61)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57, T58, T59, T60, T61, T62> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57, T58, T59, T60, T61, T62) {
    type Tuple = ((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43), T44), T45), T46), T47), T48), T49), T50), T51), T52), T53), T54), T55), T56), T57), T58), T59), T60), T61), T62);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43), t44), t45), t46), t47), t48), t49), t50), t51), t52), t53), t54), t55), t56), t57), t58), t59), t60), t61), t62) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46, t47, t48, t49, t50, t51, t52, t53, t54, t55, t56, t57, t58, t59, t60, t61, t62)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57, T58, T59, T60, T61, T62, T63> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57, T58, T59, T60, T61, T62, T63) {
    type Tuple = (((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43), T44), T45), T46), T47), T48), T49), T50), T51), T52), T53), T54), T55), T56), T57), T58), T59), T60), T61), T62), T63);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let (((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43), t44), t45), t46), t47), t48), t49), t50), t51), t52), t53), t54), t55), t56), t57), t58), t59), t60), t61), t62), t63) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46, t47, t48, t49, t50, t51, t52, t53, t54, t55, t56, t57, t58, t59, t60, t61, t62, t63)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57, T58, T59, T60, T61, T62, T63, T64> FromNestedTuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57, T58, T59, T60, T61, T62, T63, T64) {
    type Tuple = ((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((T0, T1), T2), T3), T4), T5), T6), T7), T8), T9), T10), T11), T12), T13), T14), T15), T16), T17), T18), T19), T20), T21), T22), T23), T24), T25), T26), T27), T28), T29), T30), T31), T32), T33), T34), T35), T36), T37), T38), T39), T40), T41), T42), T43), T44), T45), T46), T47), T48), T49), T50), T51), T52), T53), T54), T55), T56), T57), T58), T59), T60), T61), T62), T63), T64);

    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((t0, t1), t2), t3), t4), t5), t6), t7), t8), t9), t10), t11), t12), t13), t14), t15), t16), t17), t18), t19), t20), t21), t22), t23), t24), t25), t26), t27), t28), t29), t30), t31), t32), t33), t34), t35), t36), t37), t38), t39), t40), t41), t42), t43), t44), t45), t46), t47), t48), t49), t50), t51), t52), t53), t54), t55), t56), t57), t58), t59), t60), t61), t62), t63), t64) = tuple;
        (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46, t47, t48, t49, t50, t51, t52, t53, t54, t55, t56, t57, t58, t59, t60, t61, t62, t63, t64)
    }
}

