#![cfg_attr(feature = "native-f16", feature(f16))]

mod be_bytes;
mod bits;
mod classify;
mod cmp;
mod extend;
mod floats;
mod native;
mod pack;
mod packed;
mod repr;
mod truncate;
mod validator;

pub use self::be_bytes::{FpFromBeBytes, FpToBeBytes};
pub use self::bits::{FpFromBits, FpToBits};
pub use self::classify::FpClassify;
pub use self::extend::FpExtend;
pub use self::floats::{F16, F24, F32, F40, F48, F56, F64, F8};
pub use self::pack::FpPack;
pub use self::packed::PackedFloat;
pub use self::repr::FpRepr;
pub use self::truncate::FpTruncate;
pub use self::validator::PackedFloatValidator;

mod sealed {
    pub trait Sealed {}
}

pub(crate) use self::sealed::Sealed;

impl Sealed for F8 {}
impl Sealed for F16 {}
impl Sealed for F24 {}
impl Sealed for F32 {}
impl Sealed for F40 {}
impl Sealed for F48 {}
impl Sealed for F56 {}
impl Sealed for F64 {}

impl Sealed for PackedFloat {}

#[test]
fn foo() {
    type Target = F32;
    type Native = f32;

    let min_positive: Native = <F8 as FpExtend<Target>>::extend(F8::MIN_POSITIVE).into();
    let max: Native = <F8 as FpExtend<Target>>::extend(F8::MAX).into();
    println!("F8: {:032b} {:032b}", min_positive.to_bits(), max.to_bits());

    let min_positive: Native = <F16 as FpExtend<Target>>::extend(F16::MIN_POSITIVE).into();
    let max: Native = <F16 as FpExtend<Target>>::extend(F16::MAX).into();
    println!(
        "F16: {:032b} {:032b}",
        min_positive.to_bits(),
        max.to_bits()
    );

    let min_positive: Native = <F24 as FpExtend<Target>>::extend(F24::MIN_POSITIVE).into();
    let max: Native = <F24 as FpExtend<Target>>::extend(F24::MAX).into();
    println!(
        "F24: {:032b} {:032b}",
        min_positive.to_bits(),
        max.to_bits()
    );

    // let min_positive: Native = <F32 as FpExtend<Target>>::extend(F32::MIN_POSITIVE).into();
    // let max: Native = <F32 as FpExtend<Target>>::extend(F32::MAX).into();
    // println!(
    //     "F32: {:032b} {:032b}",
    //     min_positive.to_bits(),
    //     max.to_bits()
    // );

    // let min_positive: Native = <F40 as FpExtend<Target>>::extend(F40::MIN_POSITIVE).into();
    // let max: Native = <F40 as FpExtend<Target>>::extend(F40::MAX).into();
    // println!(
    //     "F40: {:032b} {:032b}",
    //     min_positive.to_bits(),
    //     max.to_bits()
    // );

    // let min_positive: Native = <F48 as FpExtend<Target>>::extend(F48::MIN_POSITIVE).into();
    // let max: Native = <F48 as FpExtend<Target>>::extend(F48::MAX).into();
    // println!(
    //     "F48: {:032b} {:032b}",
    //     min_positive.to_bits(),
    //     max.to_bits()
    // );

    // let min_positive: Native = <F56 as FpExtend<Target>>::extend(F56::MIN_POSITIVE).into();
    // let max: Native = <F56 as FpExtend<Target>>::extend(F56::MAX).into();
    // println!(
    //     "F56: {:032b} {:032b}",
    //     min_positive.to_bits(),
    //     max.to_bits()
    // );
}
