use crate::domain::absent::AbsentDomain;
use crate::domain::bigint::FlatBigInt;
use crate::domain::bool::FlatBool;
use crate::domain::int::FlatInt;
use crate::domain::null::NullDomain;
use crate::domain::num::FlatNum;
use crate::domain::str::SetStr;
use crate::domain::undef::UndefDomain;

pub struct BasicSimple {
    pub num: FlatNum,
    pub int: FlatInt,
    pub bigint: FlatBigInt,
    pub str: SetStr,
    pub bool: FlatBool,
    pub undef: UndefDomain,
    pub null: NullDomain,
    pub absent: AbsentDomain,
}

