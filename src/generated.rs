/// AMD ISA specification release date.
pub const ISA_RELEASE_DATE: &str = "2026-02-20";
/// AMD ISA XML schema version.
pub const ISA_SCHEMA_VERSION: &str = "1.1.1";
/// Architecture described by the generated instruction types.
pub const ISA_ARCHITECTURE: &str = "AMD RDNA 2";
pub mod predicate {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Always;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Eq;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Ge;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Gt;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Le;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Lg;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Lt;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Ne;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Never;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct NotGe;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct NotGt;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct NotLe;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct NotLg;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct NotLt;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Ordered;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Unordered;
}
impl crate::private::Compare<predicate::Always, crate::I32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Always, crate::I32, crate::E32> for () {}
impl crate::private::Compare<predicate::Always, crate::I32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Always, crate::I32, crate::E64> for () {}
impl crate::private::Compare<predicate::Always, crate::I64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Always, crate::I64, crate::E32> for () {}
impl crate::private::Compare<predicate::Always, crate::I64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Always, crate::I64, crate::E64> for () {}
impl crate::private::Compare<predicate::Always, crate::U32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Always, crate::U32, crate::E32> for () {}
impl crate::private::Compare<predicate::Always, crate::U32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Always, crate::U32, crate::E64> for () {}
impl crate::private::Compare<predicate::Always, crate::U64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Always, crate::U64, crate::E32> for () {}
impl crate::private::Compare<predicate::Always, crate::U64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Always, crate::U64, crate::E64> for () {}
impl crate::private::Compare<predicate::Eq, crate::F16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Eq, crate::F16, crate::E32> for () {}
impl crate::private::Compare<predicate::Eq, crate::F16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Eq, crate::F16, crate::E64> for () {}
impl crate::private::Compare<predicate::Eq, crate::F32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Eq, crate::F32, crate::E32> for () {}
impl crate::private::Compare<predicate::Eq, crate::F32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Eq, crate::F32, crate::E64> for () {}
impl crate::private::Compare<predicate::Eq, crate::F64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Eq, crate::F64, crate::E32> for () {}
impl crate::private::Compare<predicate::Eq, crate::F64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Eq, crate::F64, crate::E64> for () {}
impl crate::private::Compare<predicate::Eq, crate::I16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Eq, crate::I16, crate::E32> for () {}
impl crate::private::Compare<predicate::Eq, crate::I16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Eq, crate::I16, crate::E64> for () {}
impl crate::private::Compare<predicate::Eq, crate::I32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Eq, crate::I32, crate::E32> for () {}
impl crate::private::Compare<predicate::Eq, crate::I32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Eq, crate::I32, crate::E64> for () {}
impl crate::private::Compare<predicate::Eq, crate::I64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Eq, crate::I64, crate::E32> for () {}
impl crate::private::Compare<predicate::Eq, crate::I64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Eq, crate::I64, crate::E64> for () {}
impl crate::private::Compare<predicate::Eq, crate::U16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Eq, crate::U16, crate::E32> for () {}
impl crate::private::Compare<predicate::Eq, crate::U16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Eq, crate::U16, crate::E64> for () {}
impl crate::private::Compare<predicate::Eq, crate::U32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Eq, crate::U32, crate::E32> for () {}
impl crate::private::Compare<predicate::Eq, crate::U32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Eq, crate::U32, crate::E64> for () {}
impl crate::private::Compare<predicate::Eq, crate::U64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Eq, crate::U64, crate::E32> for () {}
impl crate::private::Compare<predicate::Eq, crate::U64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Eq, crate::U64, crate::E64> for () {}
impl crate::private::Compare<predicate::Ge, crate::F16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Ge, crate::F16, crate::E32> for () {}
impl crate::private::Compare<predicate::Ge, crate::F16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Ge, crate::F16, crate::E64> for () {}
impl crate::private::Compare<predicate::Ge, crate::F32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Ge, crate::F32, crate::E32> for () {}
impl crate::private::Compare<predicate::Ge, crate::F32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Ge, crate::F32, crate::E64> for () {}
impl crate::private::Compare<predicate::Ge, crate::F64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Ge, crate::F64, crate::E32> for () {}
impl crate::private::Compare<predicate::Ge, crate::F64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Ge, crate::F64, crate::E64> for () {}
impl crate::private::Compare<predicate::Ge, crate::I16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Ge, crate::I16, crate::E32> for () {}
impl crate::private::Compare<predicate::Ge, crate::I16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Ge, crate::I16, crate::E64> for () {}
impl crate::private::Compare<predicate::Ge, crate::I32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Ge, crate::I32, crate::E32> for () {}
impl crate::private::Compare<predicate::Ge, crate::I32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Ge, crate::I32, crate::E64> for () {}
impl crate::private::Compare<predicate::Ge, crate::I64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Ge, crate::I64, crate::E32> for () {}
impl crate::private::Compare<predicate::Ge, crate::I64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Ge, crate::I64, crate::E64> for () {}
impl crate::private::Compare<predicate::Ge, crate::U16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Ge, crate::U16, crate::E32> for () {}
impl crate::private::Compare<predicate::Ge, crate::U16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Ge, crate::U16, crate::E64> for () {}
impl crate::private::Compare<predicate::Ge, crate::U32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Ge, crate::U32, crate::E32> for () {}
impl crate::private::Compare<predicate::Ge, crate::U32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Ge, crate::U32, crate::E64> for () {}
impl crate::private::Compare<predicate::Ge, crate::U64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Ge, crate::U64, crate::E32> for () {}
impl crate::private::Compare<predicate::Ge, crate::U64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Ge, crate::U64, crate::E64> for () {}
impl crate::private::Compare<predicate::Gt, crate::F16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Gt, crate::F16, crate::E32> for () {}
impl crate::private::Compare<predicate::Gt, crate::F16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Gt, crate::F16, crate::E64> for () {}
impl crate::private::Compare<predicate::Gt, crate::F32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Gt, crate::F32, crate::E32> for () {}
impl crate::private::Compare<predicate::Gt, crate::F32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Gt, crate::F32, crate::E64> for () {}
impl crate::private::Compare<predicate::Gt, crate::F64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Gt, crate::F64, crate::E32> for () {}
impl crate::private::Compare<predicate::Gt, crate::F64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Gt, crate::F64, crate::E64> for () {}
impl crate::private::Compare<predicate::Gt, crate::I16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Gt, crate::I16, crate::E32> for () {}
impl crate::private::Compare<predicate::Gt, crate::I16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Gt, crate::I16, crate::E64> for () {}
impl crate::private::Compare<predicate::Gt, crate::I32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Gt, crate::I32, crate::E32> for () {}
impl crate::private::Compare<predicate::Gt, crate::I32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Gt, crate::I32, crate::E64> for () {}
impl crate::private::Compare<predicate::Gt, crate::I64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Gt, crate::I64, crate::E32> for () {}
impl crate::private::Compare<predicate::Gt, crate::I64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Gt, crate::I64, crate::E64> for () {}
impl crate::private::Compare<predicate::Gt, crate::U16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Gt, crate::U16, crate::E32> for () {}
impl crate::private::Compare<predicate::Gt, crate::U16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Gt, crate::U16, crate::E64> for () {}
impl crate::private::Compare<predicate::Gt, crate::U32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Gt, crate::U32, crate::E32> for () {}
impl crate::private::Compare<predicate::Gt, crate::U32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Gt, crate::U32, crate::E64> for () {}
impl crate::private::Compare<predicate::Gt, crate::U64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Gt, crate::U64, crate::E32> for () {}
impl crate::private::Compare<predicate::Gt, crate::U64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Gt, crate::U64, crate::E64> for () {}
impl crate::private::Compare<predicate::Le, crate::F16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Le, crate::F16, crate::E32> for () {}
impl crate::private::Compare<predicate::Le, crate::F16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Le, crate::F16, crate::E64> for () {}
impl crate::private::Compare<predicate::Le, crate::F32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Le, crate::F32, crate::E32> for () {}
impl crate::private::Compare<predicate::Le, crate::F32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Le, crate::F32, crate::E64> for () {}
impl crate::private::Compare<predicate::Le, crate::F64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Le, crate::F64, crate::E32> for () {}
impl crate::private::Compare<predicate::Le, crate::F64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Le, crate::F64, crate::E64> for () {}
impl crate::private::Compare<predicate::Le, crate::I16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Le, crate::I16, crate::E32> for () {}
impl crate::private::Compare<predicate::Le, crate::I16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Le, crate::I16, crate::E64> for () {}
impl crate::private::Compare<predicate::Le, crate::I32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Le, crate::I32, crate::E32> for () {}
impl crate::private::Compare<predicate::Le, crate::I32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Le, crate::I32, crate::E64> for () {}
impl crate::private::Compare<predicate::Le, crate::I64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Le, crate::I64, crate::E32> for () {}
impl crate::private::Compare<predicate::Le, crate::I64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Le, crate::I64, crate::E64> for () {}
impl crate::private::Compare<predicate::Le, crate::U16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Le, crate::U16, crate::E32> for () {}
impl crate::private::Compare<predicate::Le, crate::U16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Le, crate::U16, crate::E64> for () {}
impl crate::private::Compare<predicate::Le, crate::U32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Le, crate::U32, crate::E32> for () {}
impl crate::private::Compare<predicate::Le, crate::U32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Le, crate::U32, crate::E64> for () {}
impl crate::private::Compare<predicate::Le, crate::U64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Le, crate::U64, crate::E32> for () {}
impl crate::private::Compare<predicate::Le, crate::U64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Le, crate::U64, crate::E64> for () {}
impl crate::private::Compare<predicate::Lg, crate::F16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Lg, crate::F16, crate::E32> for () {}
impl crate::private::Compare<predicate::Lg, crate::F16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Lg, crate::F16, crate::E64> for () {}
impl crate::private::Compare<predicate::Lg, crate::F32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Lg, crate::F32, crate::E32> for () {}
impl crate::private::Compare<predicate::Lg, crate::F32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Lg, crate::F32, crate::E64> for () {}
impl crate::private::Compare<predicate::Lg, crate::F64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Lg, crate::F64, crate::E32> for () {}
impl crate::private::Compare<predicate::Lg, crate::F64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Lg, crate::F64, crate::E64> for () {}
impl crate::private::Compare<predicate::Lt, crate::F16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Lt, crate::F16, crate::E32> for () {}
impl crate::private::Compare<predicate::Lt, crate::F16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Lt, crate::F16, crate::E64> for () {}
impl crate::private::Compare<predicate::Lt, crate::F32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Lt, crate::F32, crate::E32> for () {}
impl crate::private::Compare<predicate::Lt, crate::F32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Lt, crate::F32, crate::E64> for () {}
impl crate::private::Compare<predicate::Lt, crate::F64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Lt, crate::F64, crate::E32> for () {}
impl crate::private::Compare<predicate::Lt, crate::F64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Lt, crate::F64, crate::E64> for () {}
impl crate::private::Compare<predicate::Lt, crate::I16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Lt, crate::I16, crate::E32> for () {}
impl crate::private::Compare<predicate::Lt, crate::I16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Lt, crate::I16, crate::E64> for () {}
impl crate::private::Compare<predicate::Lt, crate::I32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Lt, crate::I32, crate::E32> for () {}
impl crate::private::Compare<predicate::Lt, crate::I32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Lt, crate::I32, crate::E64> for () {}
impl crate::private::Compare<predicate::Lt, crate::I64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Lt, crate::I64, crate::E32> for () {}
impl crate::private::Compare<predicate::Lt, crate::I64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Lt, crate::I64, crate::E64> for () {}
impl crate::private::Compare<predicate::Lt, crate::U16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Lt, crate::U16, crate::E32> for () {}
impl crate::private::Compare<predicate::Lt, crate::U16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Lt, crate::U16, crate::E64> for () {}
impl crate::private::Compare<predicate::Lt, crate::U32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Lt, crate::U32, crate::E32> for () {}
impl crate::private::Compare<predicate::Lt, crate::U32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Lt, crate::U32, crate::E64> for () {}
impl crate::private::Compare<predicate::Lt, crate::U64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Lt, crate::U64, crate::E32> for () {}
impl crate::private::Compare<predicate::Lt, crate::U64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Lt, crate::U64, crate::E64> for () {}
impl crate::private::Compare<predicate::Ne, crate::I16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Ne, crate::I16, crate::E32> for () {}
impl crate::private::Compare<predicate::Ne, crate::I16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Ne, crate::I16, crate::E64> for () {}
impl crate::private::Compare<predicate::Ne, crate::I32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Ne, crate::I32, crate::E32> for () {}
impl crate::private::Compare<predicate::Ne, crate::I32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Ne, crate::I32, crate::E64> for () {}
impl crate::private::Compare<predicate::Ne, crate::I64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Ne, crate::I64, crate::E32> for () {}
impl crate::private::Compare<predicate::Ne, crate::I64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Ne, crate::I64, crate::E64> for () {}
impl crate::private::Compare<predicate::Ne, crate::U16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Ne, crate::U16, crate::E32> for () {}
impl crate::private::Compare<predicate::Ne, crate::U16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Ne, crate::U16, crate::E64> for () {}
impl crate::private::Compare<predicate::Ne, crate::U32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Ne, crate::U32, crate::E32> for () {}
impl crate::private::Compare<predicate::Ne, crate::U32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Ne, crate::U32, crate::E64> for () {}
impl crate::private::Compare<predicate::Ne, crate::U64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Ne, crate::U64, crate::E32> for () {}
impl crate::private::Compare<predicate::Ne, crate::U64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Ne, crate::U64, crate::E64> for () {}
impl crate::private::Compare<predicate::Never, crate::F16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Never, crate::F16, crate::E32> for () {}
impl crate::private::Compare<predicate::Never, crate::F16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Never, crate::F16, crate::E64> for () {}
impl crate::private::Compare<predicate::Never, crate::F32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Never, crate::F32, crate::E32> for () {}
impl crate::private::Compare<predicate::Never, crate::F32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Never, crate::F32, crate::E64> for () {}
impl crate::private::Compare<predicate::Never, crate::F64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Never, crate::F64, crate::E32> for () {}
impl crate::private::Compare<predicate::Never, crate::F64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Never, crate::F64, crate::E64> for () {}
impl crate::private::Compare<predicate::Never, crate::I32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Never, crate::I32, crate::E32> for () {}
impl crate::private::Compare<predicate::Never, crate::I32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Never, crate::I32, crate::E64> for () {}
impl crate::private::Compare<predicate::Never, crate::I64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Never, crate::I64, crate::E32> for () {}
impl crate::private::Compare<predicate::Never, crate::I64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Never, crate::I64, crate::E64> for () {}
impl crate::private::Compare<predicate::Never, crate::U32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Never, crate::U32, crate::E32> for () {}
impl crate::private::Compare<predicate::Never, crate::U32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Never, crate::U32, crate::E64> for () {}
impl crate::private::Compare<predicate::Never, crate::U64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Never, crate::U64, crate::E32> for () {}
impl crate::private::Compare<predicate::Never, crate::U64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Never, crate::U64, crate::E64> for () {}
impl crate::private::Compare<predicate::NotGe, crate::F16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::NotGe, crate::F16, crate::E32> for () {}
impl crate::private::Compare<predicate::NotGe, crate::F16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::NotGe, crate::F16, crate::E64> for () {}
impl crate::private::Compare<predicate::NotGe, crate::F32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::NotGe, crate::F32, crate::E32> for () {}
impl crate::private::Compare<predicate::NotGe, crate::F32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::NotGe, crate::F32, crate::E64> for () {}
impl crate::private::Compare<predicate::NotGe, crate::F64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::NotGe, crate::F64, crate::E32> for () {}
impl crate::private::Compare<predicate::NotGe, crate::F64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::NotGe, crate::F64, crate::E64> for () {}
impl crate::private::Compare<predicate::NotGt, crate::F16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::NotGt, crate::F16, crate::E32> for () {}
impl crate::private::Compare<predicate::NotGt, crate::F16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::NotGt, crate::F16, crate::E64> for () {}
impl crate::private::Compare<predicate::NotGt, crate::F32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::NotGt, crate::F32, crate::E32> for () {}
impl crate::private::Compare<predicate::NotGt, crate::F32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::NotGt, crate::F32, crate::E64> for () {}
impl crate::private::Compare<predicate::NotGt, crate::F64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::NotGt, crate::F64, crate::E32> for () {}
impl crate::private::Compare<predicate::NotGt, crate::F64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::NotGt, crate::F64, crate::E64> for () {}
impl crate::private::Compare<predicate::NotLe, crate::F16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::NotLe, crate::F16, crate::E32> for () {}
impl crate::private::Compare<predicate::NotLe, crate::F16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::NotLe, crate::F16, crate::E64> for () {}
impl crate::private::Compare<predicate::NotLe, crate::F32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::NotLe, crate::F32, crate::E32> for () {}
impl crate::private::Compare<predicate::NotLe, crate::F32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::NotLe, crate::F32, crate::E64> for () {}
impl crate::private::Compare<predicate::NotLe, crate::F64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::NotLe, crate::F64, crate::E32> for () {}
impl crate::private::Compare<predicate::NotLe, crate::F64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::NotLe, crate::F64, crate::E64> for () {}
impl crate::private::Compare<predicate::NotLg, crate::F16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::NotLg, crate::F16, crate::E32> for () {}
impl crate::private::Compare<predicate::NotLg, crate::F16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::NotLg, crate::F16, crate::E64> for () {}
impl crate::private::Compare<predicate::NotLg, crate::F32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::NotLg, crate::F32, crate::E32> for () {}
impl crate::private::Compare<predicate::NotLg, crate::F32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::NotLg, crate::F32, crate::E64> for () {}
impl crate::private::Compare<predicate::NotLg, crate::F64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::NotLg, crate::F64, crate::E32> for () {}
impl crate::private::Compare<predicate::NotLg, crate::F64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::NotLg, crate::F64, crate::E64> for () {}
impl crate::private::Compare<predicate::NotLt, crate::F16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::NotLt, crate::F16, crate::E32> for () {}
impl crate::private::Compare<predicate::NotLt, crate::F16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::NotLt, crate::F16, crate::E64> for () {}
impl crate::private::Compare<predicate::NotLt, crate::F32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::NotLt, crate::F32, crate::E32> for () {}
impl crate::private::Compare<predicate::NotLt, crate::F32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::NotLt, crate::F32, crate::E64> for () {}
impl crate::private::Compare<predicate::NotLt, crate::F64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::NotLt, crate::F64, crate::E32> for () {}
impl crate::private::Compare<predicate::NotLt, crate::F64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::NotLt, crate::F64, crate::E64> for () {}
impl crate::private::Compare<predicate::Ordered, crate::F16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Ordered, crate::F16, crate::E32> for () {}
impl crate::private::Compare<predicate::Ordered, crate::F16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Ordered, crate::F16, crate::E64> for () {}
impl crate::private::Compare<predicate::Ordered, crate::F32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Ordered, crate::F32, crate::E32> for () {}
impl crate::private::Compare<predicate::Ordered, crate::F32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Ordered, crate::F32, crate::E64> for () {}
impl crate::private::Compare<predicate::Ordered, crate::F64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Ordered, crate::F64, crate::E32> for () {}
impl crate::private::Compare<predicate::Ordered, crate::F64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Ordered, crate::F64, crate::E64> for () {}
impl crate::private::Compare<predicate::Unordered, crate::F16, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Unordered, crate::F16, crate::E32> for () {}
impl crate::private::Compare<predicate::Unordered, crate::F16, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Unordered, crate::F16, crate::E64> for () {}
impl crate::private::Compare<predicate::Unordered, crate::F32, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Unordered, crate::F32, crate::E32> for () {}
impl crate::private::Compare<predicate::Unordered, crate::F32, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Unordered, crate::F32, crate::E64> for () {}
impl crate::private::Compare<predicate::Unordered, crate::F64, crate::E32> for () {}
impl crate::SupportedCompare<predicate::Unordered, crate::F64, crate::E32> for () {}
impl crate::private::Compare<predicate::Unordered, crate::F64, crate::E64> for () {}
impl crate::SupportedCompare<predicate::Unordered, crate::F64, crate::E64> for () {}
pub mod family {
    /// `v_add_f16`: Add two floating point inputs and store the result into a vector register.
    ///
    /// `v_add_f32`: Add two floating point inputs and store the result into a vector register.
    ///
    /// `v_add_f64`: Add two floating point inputs and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VAdd;
    /// `v_add3_u32`: Add three unsigned inputs and store the result into a vector register. No carry-in or carry-out support.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VAdd3;
    /// `v_add_lshl_u32`: Add the first two integer inputs, then given a shift count in the third input, calculate the logical shift left of the intermediate result, then store the final result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VAddLshl;
    /// `v_add_nc_i16`: Add two signed 16-bit integer inputs and store the result into a vector register. No carry-in or carry-out support.
    ///
    /// `v_add_nc_i32`: Add two signed 32-bit integer inputs and store the result into a vector register. No carry-in or carry-out support.
    ///
    /// `v_add_nc_u16`: Add two unsigned 16-bit integer inputs and store the result into a vector register. No carry-in or carry-out support.
    ///
    /// `v_add_nc_u32`: Add two unsigned 32-bit integer inputs and store the result into a vector register. No carry-in or carry-out support.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VAddNc;
    /// `v_alignbit_b32`: Align a 64-bit value encoded in the first two inputs to a bit position specified in the third input, then store the result into a 32-bit vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VAlignbit;
    /// `v_alignbyte_b32`: Align a 64-bit value encoded in the first two inputs to a byte position specified in the third input, then store the result into a 32-bit vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VAlignbyte;
    /// `v_and_b32`: Calculate bitwise AND on two vector inputs and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VAnd;
    /// `v_and_or_b32`: Calculate bitwise AND on the first two vector inputs, then compute the bitwise OR of the intermediate result and the third vector input, then store the final result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VAndOr;
    /// `v_ashrrev_i16`: Given a shift count in the first vector input, calculate the arithmetic shift right (preserving sign bit) of the second vector input and store the result into a vector register.
    ///
    /// `v_ashrrev_i32`: Given a shift count in the first vector input, calculate the arithmetic shift right (preserving sign bit) of the second vector input and store the result into a vector register.
    ///
    /// `v_ashrrev_i64`: Given a shift count in the first vector input, calculate the arithmetic shift right (preserving sign bit) of the second vector input and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VAshrrev;
    /// `v_bcnt_u32_b32`: Count the number of "1" bits in the vector input and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VBcnt;
    /// `v_bfe_i32`: Extract a signed bitfield from the first input using field offset from the second input and size from the third input, then store the result into a vector register.
    ///
    /// `v_bfe_u32`: Extract an unsigned bitfield from the first input using field offset from the second input and size from the third input, then store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VBfe;
    /// `v_bfi_b32`: Overwrite a bitfield in the third input with a bitfield from the second input using a mask from the first input, then store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VBfi;
    /// `v_bfm_b32`: Calculate a bitfield mask given a field offset and size and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VBfm;
    /// `v_bfrev_b32`: Reverse the order of bits in a vector input and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VBfrev;
    /// `v_ceil_f16`: Round the half-precision float input up to next integer and store the result in floating point format into a vector register.
    ///
    /// `v_ceil_f32`: Round the single-precision float input up to next integer and store the result in floating point format into a vector register.
    ///
    /// `v_ceil_f64`: Round the double-precision float input up to next integer and store the result in floating point format into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VCeil;
    /// `v_cos_f16`: Calculate the trigonometric cosine of a half-precision float value using IEEE rules and store the result into a vector register. The operand is calculated by scaling the vector input by 2 PI.
    ///
    /// `v_cos_f32`: Calculate the trigonometric cosine of a single-precision float value using IEEE rules and store the result into a vector register. The operand is calculated by scaling the vector input by 2 PI.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VCos;
    /// `v_cubeid_f32`: Compute the cubemap face ID of a 3D coordinate specified as three single-precision float inputs. Store the result in single-precision float format into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VCubeid;
    /// `v_cubema_f32`: Compute the cubemap major axis of a 3D coordinate specified as three single-precision float inputs. Store the result in single-precision float format into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VCubema;
    /// `v_cubesc_f32`: Compute the cubemap S coordinate of a 3D coordinate specified as three single-precision float inputs. Store the result in single-precision float format into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VCubesc;
    /// `v_cubetc_f32`: Compute the cubemap T coordinate of a 3D coordinate specified as three single-precision float inputs. Store the result in single-precision float format into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VCubetc;
    /// `v_cvt_f16_f32`: Convert from a single-precision float input to a half-precision float value and store the result into a vector register.
    ///
    /// `v_cvt_f16_i16`: Convert from a signed 16-bit integer input to a half-precision float value and store the result into a vector register.
    ///
    /// `v_cvt_f16_u16`: Convert from an unsigned 16-bit integer input to a half-precision float value and store the result into a vector register.
    ///
    /// `v_cvt_f32_f16`: Convert from a half-precision float input to a single-precision float value and store the result into a vector register.
    ///
    /// `v_cvt_f32_f64`: Convert from a double-precision float input to a single-precision float value and store the result into a vector register.
    ///
    /// `v_cvt_f32_i32`: Convert from a signed 32-bit integer input to a single-precision float value and store the result into a vector register.
    ///
    /// `v_cvt_f32_u32`: Convert from an unsigned 32-bit integer input to a single-precision float value and store the result into a vector register.
    ///
    /// `v_cvt_f64_f32`: Convert from a single-precision float input to a double-precision float value and store the result into a vector register.
    ///
    /// `v_cvt_f64_i32`: Convert from a signed 32-bit integer input to a double-precision float value and store the result into a vector register.
    ///
    /// `v_cvt_f64_u32`: Convert from an unsigned 32-bit integer input to a double-precision float value and store the result into a vector register.
    ///
    /// `v_cvt_i16_f16`: Convert from a half-precision float input to a signed 16-bit integer value and store the result into a vector register.
    ///
    /// `v_cvt_i32_f32`: Convert from a single-precision float input to a signed 32-bit integer value and store the result into a vector register.
    ///
    /// `v_cvt_i32_f64`: Convert from a double-precision float input to a signed 32-bit integer value and store the result into a vector register.
    ///
    /// `v_cvt_u16_f16`: Convert from a half-precision float input to an unsigned 16-bit integer value and store the result into a vector register.
    ///
    /// `v_cvt_u32_f32`: Convert from a single-precision float input to an unsigned 32-bit integer value and store the result into a vector register.
    ///
    /// `v_cvt_u32_f64`: Convert from a double-precision float input to an unsigned 32-bit integer value and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VCvt;
    /// `v_cvt_flr_i32_f32`: Convert from a single-precision float input to a signed 32-bit integer value using round-down semantics (ignore the default rounding mode) and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VCvtFlr;
    /// `v_cvt_norm_i16_f16`: Convert from a half-precision float input to a signed normalized short and store the result into a vector register.
    ///
    /// `v_cvt_norm_u16_f16`: Convert from a half-precision float input to an unsigned normalized short and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VCvtNorm;
    /// `v_cvt_rpi_i32_f32`: Convert from a single-precision float input to a signed 32-bit integer value using round to nearest integer semantics (ignore the default rounding mode) and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VCvtRpi;
    /// `v_div_fixup_f16`: Given a half-precision float quotient in the first input, a denominator in the second input and a numerator in the third input, detect and apply corner cases related to division, including divide by zero, NaN inputs and overflow, and modify the quotient accordingly. Generate any invalid, denormal and divide-by-zero exceptions that are a result of the division. Store the modified quotient into a vector register.
    ///
    /// `v_div_fixup_f32`: Given a single-precision float quotient in the first input, a denominator in the second input and a numerator in the third input, detect and apply corner cases related to division, including divide by zero, NaN inputs and overflow, and modify the quotient accordingly. Generate any invalid, denormal and divide-by-zero exceptions that are a result of the division. Store the modified quotient into a vector register.
    ///
    /// `v_div_fixup_f64`: Given a double-precision float quotient in the first input, a denominator in the second input and a numerator in the third input, detect and apply corner cases related to division, including divide by zero, NaN inputs and overflow, and modify the quotient accordingly. Generate any invalid, denormal and divide-by-zero exceptions that are a result of the division. Store the modified quotient into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VDivFixup;
    /// `v_exp_f16`: Calculate 2 raised to the power of the half-precision float input and store the result into a vector register.
    ///
    /// `v_exp_f32`: Calculate 2 raised to the power of the single-precision float input and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VExp;
    /// `v_ffbh_i32`: Count the number of leading bits that are the same as the sign bit of a vector input and store the result into a vector register. Store -1 if all input bits are the same.
    ///
    /// `v_ffbh_u32`: Count the number of leading "0" bits before the first "1" in a vector input and store the result into a vector register. Store -1 if there are no "1" bits.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VFfbh;
    /// `v_ffbl_b32`: Count the number of trailing "0" bits before the first "1" in a vector input and store the result into a vector register. Store -1 if there are no "1" bits in the input.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VFfbl;
    /// `v_floor_f16`: Round the half-precision float input down to previous integer and store the result in floating point format into a vector register.
    ///
    /// `v_floor_f32`: Round the single-precision float input down to previous integer and store the result in floating point format into a vector register.
    ///
    /// `v_floor_f64`: Round the double-precision float input down to previous integer and store the result in floating point format into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VFloor;
    /// `v_fma_f16`: Multiply two half-precision float inputs and add a third input using fused multiply add, and store the result into a vector register.
    ///
    /// `v_fma_f32`: Multiply two single-precision float inputs and add a third input using fused multiply add, and store the result into a vector register.
    ///
    /// `v_fma_f64`: Multiply two double-precision float inputs and add a third input using fused multiply add, and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VFma;
    /// `v_fma_legacy_f32`: Multiply and add single-precision values. Follows DX9 rules where 0.0 times anything produces 0.0.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VFmaLegacy;
    /// `v_fmac_f16`: Multiply two half-precision float inputs and accumulate the result into the destination register using fused multiply add.
    ///
    /// `v_fmac_f32`: Multiply two single-precision float inputs and accumulate the result into the destination register using fused multiply add.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VFmac;
    /// `v_fmac_legacy_f32`: Multiply two single-precision values and accumulate the result with the destination. Follows DX9 rules where 0.0 times anything produces 0.0.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VFmacLegacy;
    /// `v_fract_f16`: Compute the fractional portion of a half-precision float input and store the result in floating point format into a vector register.
    ///
    /// `v_fract_f32`: Compute the fractional portion of a single-precision float input and store the result in floating point format into a vector register.
    ///
    /// `v_fract_f64`: Compute the fractional portion of a double-precision float input and store the result in floating point format into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VFract;
    /// `v_frexp_exp_i16_f16`: Extract the exponent of a half-precision float input and store the result as a signed 16-bit integer into a vector register.
    ///
    /// `v_frexp_exp_i32_f32`: Extract the exponent of a single-precision float input and store the result as a signed 32-bit integer into a vector register.
    ///
    /// `v_frexp_exp_i32_f64`: Extract the exponent of a double-precision float input and store the result as a signed 32-bit integer into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VFrexpExp;
    /// `v_frexp_mant_f16`: Extract the binary significand, or mantissa, of a half-precision float input and store the result as a half-precision float into a vector register.
    ///
    /// `v_frexp_mant_f32`: Extract the binary significand, or mantissa, of a single-precision float input and store the result as a single-precision float into a vector register.
    ///
    /// `v_frexp_mant_f64`: Extract the binary significand, or mantissa, of a double-precision float input and store the result as a double-precision float into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VFrexpMant;
    /// `v_ldexp_f16`: Multiply the first input, a floating point value, by an integral power of 2 specified in the second input, a signed integer value, and store the floating point result into a vector register.
    ///
    /// `v_ldexp_f32`: Multiply the first input, a floating point value, by an integral power of 2 specified in the second input, a signed integer value, and store the floating point result into a vector register.
    ///
    /// `v_ldexp_f64`: Multiply the first input, a floating point value, by an integral power of 2 specified in the second input, a signed integer value, and store the floating point result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VLdexp;
    /// `v_lerp_u8`: Average two 4-D vectors stored as packed bytes in the first two inputs with rounding control provided by the third input, then store the result into a vector register. Each byte in the third input acts as a rounding mode for the corresponding element; if the LSB is set then 0.5 rounds up, otherwise 0.5 truncates.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VLerp;
    /// `v_log_f16`: Calculate the base 2 logarithm of the half-precision float input and store the result into a vector register.
    ///
    /// `v_log_f32`: Calculate the base 2 logarithm of the single-precision float input and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VLog;
    /// `v_lshl_add_u32`: Given a shift count in the second input, calculate the logical shift left of the first input, then add the third input to the intermediate result, then store the final result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VLshlAdd;
    /// `v_lshl_or_b32`: Given a shift count in the second input, calculate the logical shift left of the first input, then calculate the bitwise OR of the intermediate result and the third input, then store the final result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VLshlOr;
    /// `v_lshlrev_b16`: Given a shift count in the first vector input, calculate the logical shift left of the second vector input and store the result into a vector register.
    ///
    /// `v_lshlrev_b32`: Given a shift count in the first vector input, calculate the logical shift left of the second vector input and store the result into a vector register.
    ///
    /// `v_lshlrev_b64`: Given a shift count in the first vector input, calculate the logical shift left of the second vector input and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VLshlrev;
    /// `v_lshrrev_b16`: Given a shift count in the first vector input, calculate the logical shift right of the second vector input and store the result into a vector register.
    ///
    /// `v_lshrrev_b32`: Given a shift count in the first vector input, calculate the logical shift right of the second vector input and store the result into a vector register.
    ///
    /// `v_lshrrev_b64`: Given a shift count in the first vector input, calculate the logical shift right of the second vector input and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VLshrrev;
    /// `v_mad_i16`: Multiply two signed 16-bit integer inputs, add a signed 16-bit integer value from a third input, and store the result into a vector register.
    ///
    /// `v_mad_i32_i16`: Multiply two signed 16-bit integer inputs in the signed 32-bit integer domain, add a signed 32-bit integer value from a third input, and store the result as a signed 32-bit integer into a vector register.
    ///
    /// `v_mad_u16`: Multiply two unsigned 16-bit integer inputs, add an unsigned 16-bit integer value from a third input, and store the result into a vector register.
    ///
    /// `v_mad_u32_u16`: Multiply two unsigned 16-bit integer inputs in the unsigned 32-bit integer domain, add an unsigned 32-bit integer value from a third input, and store the result as an unsigned 32-bit integer into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VMad;
    /// `v_max_f16`: Select the maximum of two half-precision float inputs and store the selected value into a vector register.
    ///
    /// `v_max_f32`: Select the maximum of two single-precision float inputs and store the selected value into a vector register.
    ///
    /// `v_max_f64`: Select the maximum of two double-precision float inputs and store the selected value into a vector register.
    ///
    /// `v_max_i16`: Select the maximum of two signed 16-bit integer inputs and store the selected value into a vector register.
    ///
    /// `v_max_i32`: Select the maximum of two signed 32-bit integer inputs and store the selected value into a vector register.
    ///
    /// `v_max_u16`: Select the maximum of two unsigned 16-bit integer inputs and store the selected value into a vector register.
    ///
    /// `v_max_u32`: Select the maximum of two unsigned 32-bit integer inputs and store the selected value into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VMax;
    /// `v_max3_f16`: Select the maximum of three half-precision float inputs and store the selected value into a vector register.
    ///
    /// `v_max3_f32`: Select the maximum of three single-precision float inputs and store the selected value into a vector register.
    ///
    /// `v_max3_i16`: Select the maximum of three signed 16-bit integer inputs and store the selected value into a vector register.
    ///
    /// `v_max3_i32`: Select the maximum of three signed 32-bit integer inputs and store the selected value into a vector register.
    ///
    /// `v_max3_u16`: Select the maximum of three unsigned 16-bit integer inputs and store the selected value into a vector register.
    ///
    /// `v_max3_u32`: Select the maximum of three unsigned 32-bit integer inputs and store the selected value into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VMax3;
    /// `v_mbcnt_hi_u32_b32`: For each lane 32 <= N < 64, examine the N least significant bits of the first input and count how many of those bits are "1". For lane positions 0 <= N < 32 no bits are examined and the count is zero. Add this count to the value in the second input and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VMbcntHi;
    /// `v_mbcnt_lo_u32_b32`: For each lane 0 <= N < 32, examine the N least significant bits of the first input and count how many of those bits are "1". For each lane 32 <= N < 64, all "1" bits in the first input are counted. Add this count to the value in the second input and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VMbcntLo;
    /// `v_med3_f16`: Select the median of three half-precision float inputs and store the selected value into a vector register.
    ///
    /// `v_med3_f32`: Select the median of three single-precision float inputs and store the selected value into a vector register.
    ///
    /// `v_med3_i16`: Select the median of three signed 16-bit integer inputs and store the selected value into a vector register.
    ///
    /// `v_med3_i32`: Select the median of three signed 32-bit integer inputs and store the selected value into a vector register.
    ///
    /// `v_med3_u16`: Select the median of three unsigned 16-bit integer inputs and store the selected value into a vector register.
    ///
    /// `v_med3_u32`: Select the median of three unsigned 32-bit integer inputs and store the selected value into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VMed3;
    /// `v_min_f16`: Select the minimum of two half-precision float inputs and store the selected value into a vector register.
    ///
    /// `v_min_f32`: Select the minimum of two single-precision float inputs and store the selected value into a vector register.
    ///
    /// `v_min_f64`: Select the minimum of two double-precision float inputs and store the selected value into a vector register.
    ///
    /// `v_min_i16`: Select the minimum of two signed 16-bit integer inputs and store the selected value into a vector register.
    ///
    /// `v_min_i32`: Select the minimum of two signed 32-bit integer inputs and store the selected value into a vector register.
    ///
    /// `v_min_u16`: Select the minimum of two unsigned 16-bit integer inputs and store the selected value into a vector register.
    ///
    /// `v_min_u32`: Select the minimum of two unsigned 32-bit integer inputs and store the selected value into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VMin;
    /// `v_min3_f16`: Select the minimum of three half-precision float inputs and store the selected value into a vector register.
    ///
    /// `v_min3_f32`: Select the minimum of three single-precision float inputs and store the selected value into a vector register.
    ///
    /// `v_min3_i16`: Select the minimum of three signed 16-bit integer inputs and store the selected value into a vector register.
    ///
    /// `v_min3_i32`: Select the minimum of three signed 32-bit integer inputs and store the selected value into a vector register.
    ///
    /// `v_min3_u16`: Select the minimum of three unsigned 16-bit integer inputs and store the selected value into a vector register.
    ///
    /// `v_min3_u32`: Select the minimum of three unsigned 32-bit integer inputs and store the selected value into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VMin3;
    /// `v_mov_b32`: Move 32-bit data from a vector input into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VMov;
    /// `v_mqsad_pk_u16_u8`: Perform the V_MSAD_U8 operation four times using different slices of the first array, all entries of the second array and each entry of the third array. Truncate each result to 16 bits, pack the values into a 4-entry array and store the array into a vector register. The first input is an 8-entry array of unsigned 8-bit integers, the second input is a 4-entry array of unsigned 8-bit integers and the third input is a 4-entry array of unsigned 16-bit integers.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VMqsadPk;
    /// `v_msad_u8`: Calculate the sum of absolute differences of elements in two packed 4-component unsigned 8-bit integer inputs, except that elements where the second input (known as the reference input) is zero are not included in the sum. Add an unsigned 32-bit integer value from the third input and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VMsad;
    /// `v_mul_f16`: Multiply two floating point inputs and store the result into a vector register.
    ///
    /// `v_mul_f32`: Multiply two floating point inputs and store the result into a vector register.
    ///
    /// `v_mul_f64`: Multiply two floating point inputs and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VMul;
    /// `v_mul_hi_i32`: Multiply two signed 32-bit integer inputs and store the high 32 bits of the result into a vector register.
    ///
    /// `v_mul_hi_u32`: Multiply two unsigned 32-bit integer inputs and store the high 32 bits of the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VMulHi;
    /// `v_mul_legacy_f32`: Multiply two floating point inputs and store the result into a vector register. Follows DX9 rules where 0.0 times anything produces 0.0 (this differs from other APIs when the other input is infinity or NaN).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VMulLegacy;
    /// `v_mul_lo_u16`: Multiply two unsigned 16-bit integer inputs and store the low bits of the result into a vector register.
    ///
    /// `v_mul_lo_u32`: Multiply two unsigned 32-bit integer inputs and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VMulLo;
    /// `v_mullit_f32`: Multiply two floating point inputs and store the result into a vector register. Specific rules apply to accommodate lighting calculations: 0.0 * x = 0.0 and alternate INF, NAN, overflow rules apply.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VMullit;
    /// `v_not_b32`: Calculate bitwise negation on a vector input and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VNot;
    /// `v_or_b32`: Calculate bitwise OR on two vector inputs and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VOr;
    /// `v_or3_b32`: Calculate the bitwise OR of three vector inputs and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VOr3;
    /// `v_pack_b32_f16`: Pack two half-precision float values into a single 32-bit value and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VPack;
    /// `v_qsad_pk_u16_u8`: Perform the V_SAD_U8 operation four times using different slices of the first array, all entries of the second array and each entry of the third array. Truncate each result to 16 bits, pack the values into a 4-entry array and store the array into a vector register. The first input is an 8-entry array of unsigned 8-bit integers, the second input is a 4-entry array of unsigned 8-bit integers and the third input is a 4-entry array of unsigned 16-bit integers.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VQsadPk;
    /// `v_rcp_f16`: Calculate the reciprocal of the half-precision float input using IEEE rules and store the result into a vector register.
    ///
    /// `v_rcp_f32`: Calculate the reciprocal of the single-precision float input using IEEE rules and store the result into a vector register.
    ///
    /// `v_rcp_f64`: Calculate the reciprocal of the double-precision float input using IEEE rules and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VRcp;
    /// `v_rcp_iflag_f32`: Calculate the reciprocal of the vector float input in a manner suitable for integer division and store the result into a vector register. This opcode is intended for use as part of an integer division macro.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VRcpIflag;
    /// `v_rndne_f16`: Round the half-precision float input to the nearest even integer and store the result in floating point format into a vector register.
    ///
    /// `v_rndne_f32`: Round the single-precision float input to the nearest even integer and store the result in floating point format into a vector register.
    ///
    /// `v_rndne_f64`: Round the double-precision float input to the nearest even integer and store the result in floating point format into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VRndne;
    /// `v_rsq_f16`: Calculate the reciprocal of the square root of the half-precision float input using IEEE rules and store the result into a vector register.
    ///
    /// `v_rsq_f32`: Calculate the reciprocal of the square root of the single-precision float input using IEEE rules and store the result into a vector register.
    ///
    /// `v_rsq_f64`: Calculate the reciprocal of the square root of the double-precision float input using IEEE rules and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VRsq;
    /// `v_sad_u32`: Calculate the absolute difference of two unsigned 32-bit integer inputs, add an unsigned 32-bit integer value from the third input and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VSad;
    /// `v_sin_f16`: Calculate the trigonometric sine of a half-precision float value using IEEE rules and store the result into a vector register. The operand is calculated by scaling the vector input by 2 PI.
    ///
    /// `v_sin_f32`: Calculate the trigonometric sine of a single-precision float value using IEEE rules and store the result into a vector register. The operand is calculated by scaling the vector input by 2 PI.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VSin;
    /// `v_sqrt_f16`: Calculate the square root of the half-precision float input using IEEE rules and store the result into a vector register.
    ///
    /// `v_sqrt_f32`: Calculate the square root of the single-precision float input using IEEE rules and store the result into a vector register.
    ///
    /// `v_sqrt_f64`: Calculate the square root of the double-precision float input using IEEE rules and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VSqrt;
    /// `v_sub_f16`: Subtract the second floating point input from the first input and store the result into a vector register.
    ///
    /// `v_sub_f32`: Subtract the second floating point input from the first input and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VSub;
    /// `v_sub_nc_i16`: Subtract the second signed 16-bit integer input from the first input and store the result into a vector register. No carry-in or carry-out support.
    ///
    /// `v_sub_nc_i32`: Subtract the second signed 32-bit integer input from the first input and store the result into a vector register. No carry-in or carry-out support.
    ///
    /// `v_sub_nc_u16`: Subtract the second unsigned 16-bit integer input from the first input and store the result into a vector register. No carry-in or carry-out support.
    ///
    /// `v_sub_nc_u32`: Subtract the second unsigned 32-bit integer input from the first input and store the result into a vector register. No carry-in or carry-out support.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VSubNc;
    /// `v_subrev_f16`: Subtract the first floating point input from the second input and store the result into a vector register.
    ///
    /// `v_subrev_f32`: Subtract the first floating point input from the second input and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VSubrev;
    /// `v_subrev_nc_u32`: Subtract the first unsigned 32-bit integer input from the second input and store the result into a vector register. No carry-in or carry-out support.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VSubrevNc;
    /// `v_trig_preop_f64`: Look up a 53-bit segment of 2/PI using an integer segment select in the second input. Scale the intermediate result by the exponent from the first double-precision float input and store the double-precision float result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VTrigPreop;
    /// `v_trunc_f16`: Compute the integer part of a half-precision float input using round toward zero semantics and store the result in floating point format into a vector register.
    ///
    /// `v_trunc_f32`: Compute the integer part of a single-precision float input using round toward zero semantics and store the result in floating point format into a vector register.
    ///
    /// `v_trunc_f64`: Compute the integer part of a double-precision float input using round toward zero semantics and store the result in floating point format into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VTrunc;
    /// `v_xad_u32`: Calculate bitwise XOR of the first two vector inputs, then add the third vector input to the intermediate result, then store the final result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VXad;
    /// `v_xnor_b32`: Calculate bitwise XNOR on two vector inputs and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VXnor;
    /// `v_xor_b32`: Calculate bitwise XOR on two vector inputs and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VXor;
    /// `v_xor3_b32`: Calculate the bitwise XOR of three vector inputs and store the result into a vector register.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VXor3;
}
/// `v_add_f16`: Add two floating point inputs and store the result into a vector register.
///
/// `v_add_f32`: Add two floating point inputs and store the result into a vector register.
///
/// `v_add_f64`: Add two floating point inputs and store the result into a vector register.
pub type VAdd<T, E> = crate::VectorBinary<family::VAdd, T, T, T, E>;
/// `v_add3_u32`: Add three unsigned inputs and store the result into a vector register. No carry-in or carry-out support.
pub type VAdd3<T, E> = crate::VectorTernary<family::VAdd3, T, T, T, T, E>;
/// `v_add_lshl_u32`: Add the first two integer inputs, then given a shift count in the third input, calculate the logical shift left of the intermediate result, then store the final result into a vector register.
pub type VAddLshl<T, E> = crate::VectorTernary<family::VAddLshl, T, T, T, T, E>;
/// `v_add_nc_i16`: Add two signed 16-bit integer inputs and store the result into a vector register. No carry-in or carry-out support.
///
/// `v_add_nc_i32`: Add two signed 32-bit integer inputs and store the result into a vector register. No carry-in or carry-out support.
///
/// `v_add_nc_u16`: Add two unsigned 16-bit integer inputs and store the result into a vector register. No carry-in or carry-out support.
///
/// `v_add_nc_u32`: Add two unsigned 32-bit integer inputs and store the result into a vector register. No carry-in or carry-out support.
pub type VAddNc<T, E> = crate::VectorBinary<family::VAddNc, T, T, T, E>;
/// `v_alignbit_b32`: Align a 64-bit value encoded in the first two inputs to a bit position specified in the third input, then store the result into a 32-bit vector register.
pub type VAlignbit<D, S0, S1, S2, E> = crate::VectorTernary<family::VAlignbit, D, S0, S1, S2, E>;
/// `v_alignbyte_b32`: Align a 64-bit value encoded in the first two inputs to a byte position specified in the third input, then store the result into a 32-bit vector register.
pub type VAlignbyte<D, S0, S1, S2, E> = crate::VectorTernary<family::VAlignbyte, D, S0, S1, S2, E>;
/// `v_and_b32`: Calculate bitwise AND on two vector inputs and store the result into a vector register.
pub type VAnd<T, E> = crate::VectorBinary<family::VAnd, T, T, T, E>;
/// `v_and_or_b32`: Calculate bitwise AND on the first two vector inputs, then compute the bitwise OR of the intermediate result and the third vector input, then store the final result into a vector register.
pub type VAndOr<T, E> = crate::VectorTernary<family::VAndOr, T, T, T, T, E>;
/// `v_ashrrev_i16`: Given a shift count in the first vector input, calculate the arithmetic shift right (preserving sign bit) of the second vector input and store the result into a vector register.
///
/// `v_ashrrev_i32`: Given a shift count in the first vector input, calculate the arithmetic shift right (preserving sign bit) of the second vector input and store the result into a vector register.
///
/// `v_ashrrev_i64`: Given a shift count in the first vector input, calculate the arithmetic shift right (preserving sign bit) of the second vector input and store the result into a vector register.
pub type VAshrrev<D, S0, S1, E> = crate::VectorBinary<family::VAshrrev, D, S0, S1, E>;
/// `v_bcnt_u32_b32`: Count the number of "1" bits in the vector input and store the result into a vector register.
pub type VBcnt<T, E> = crate::VectorBinary<family::VBcnt, T, T, T, E>;
/// `v_bfe_i32`: Extract a signed bitfield from the first input using field offset from the second input and size from the third input, then store the result into a vector register.
///
/// `v_bfe_u32`: Extract an unsigned bitfield from the first input using field offset from the second input and size from the third input, then store the result into a vector register.
pub type VBfe<D, S0, S1, S2, E> = crate::VectorTernary<family::VBfe, D, S0, S1, S2, E>;
/// `v_bfi_b32`: Overwrite a bitfield in the third input with a bitfield from the second input using a mask from the first input, then store the result into a vector register.
pub type VBfi<T, E> = crate::VectorTernary<family::VBfi, T, T, T, T, E>;
/// `v_bfm_b32`: Calculate a bitfield mask given a field offset and size and store the result into a vector register.
pub type VBfm<T, E> = crate::VectorBinary<family::VBfm, T, T, T, E>;
/// `v_bfrev_b32`: Reverse the order of bits in a vector input and store the result into a vector register.
pub type VBfrev<T, E> = crate::VectorUnary<family::VBfrev, T, T, E>;
/// `v_ceil_f16`: Round the half-precision float input up to next integer and store the result in floating point format into a vector register.
///
/// `v_ceil_f32`: Round the single-precision float input up to next integer and store the result in floating point format into a vector register.
///
/// `v_ceil_f64`: Round the double-precision float input up to next integer and store the result in floating point format into a vector register.
pub type VCeil<T, E> = crate::VectorUnary<family::VCeil, T, T, E>;
/// `v_cos_f16`: Calculate the trigonometric cosine of a half-precision float value using IEEE rules and store the result into a vector register. The operand is calculated by scaling the vector input by 2 PI.
///
/// `v_cos_f32`: Calculate the trigonometric cosine of a single-precision float value using IEEE rules and store the result into a vector register. The operand is calculated by scaling the vector input by 2 PI.
pub type VCos<T, E> = crate::VectorUnary<family::VCos, T, T, E>;
/// `v_cubeid_f32`: Compute the cubemap face ID of a 3D coordinate specified as three single-precision float inputs. Store the result in single-precision float format into a vector register.
pub type VCubeid<T, E> = crate::VectorTernary<family::VCubeid, T, T, T, T, E>;
/// `v_cubema_f32`: Compute the cubemap major axis of a 3D coordinate specified as three single-precision float inputs. Store the result in single-precision float format into a vector register.
pub type VCubema<T, E> = crate::VectorTernary<family::VCubema, T, T, T, T, E>;
/// `v_cubesc_f32`: Compute the cubemap S coordinate of a 3D coordinate specified as three single-precision float inputs. Store the result in single-precision float format into a vector register.
pub type VCubesc<T, E> = crate::VectorTernary<family::VCubesc, T, T, T, T, E>;
/// `v_cubetc_f32`: Compute the cubemap T coordinate of a 3D coordinate specified as three single-precision float inputs. Store the result in single-precision float format into a vector register.
pub type VCubetc<T, E> = crate::VectorTernary<family::VCubetc, T, T, T, T, E>;
/// `v_cvt_f16_f32`: Convert from a single-precision float input to a half-precision float value and store the result into a vector register.
///
/// `v_cvt_f16_i16`: Convert from a signed 16-bit integer input to a half-precision float value and store the result into a vector register.
///
/// `v_cvt_f16_u16`: Convert from an unsigned 16-bit integer input to a half-precision float value and store the result into a vector register.
///
/// `v_cvt_f32_f16`: Convert from a half-precision float input to a single-precision float value and store the result into a vector register.
///
/// `v_cvt_f32_f64`: Convert from a double-precision float input to a single-precision float value and store the result into a vector register.
///
/// `v_cvt_f32_i32`: Convert from a signed 32-bit integer input to a single-precision float value and store the result into a vector register.
///
/// `v_cvt_f32_u32`: Convert from an unsigned 32-bit integer input to a single-precision float value and store the result into a vector register.
///
/// `v_cvt_f64_f32`: Convert from a single-precision float input to a double-precision float value and store the result into a vector register.
///
/// `v_cvt_f64_i32`: Convert from a signed 32-bit integer input to a double-precision float value and store the result into a vector register.
///
/// `v_cvt_f64_u32`: Convert from an unsigned 32-bit integer input to a double-precision float value and store the result into a vector register.
///
/// `v_cvt_i16_f16`: Convert from a half-precision float input to a signed 16-bit integer value and store the result into a vector register.
///
/// `v_cvt_i32_f32`: Convert from a single-precision float input to a signed 32-bit integer value and store the result into a vector register.
///
/// `v_cvt_i32_f64`: Convert from a double-precision float input to a signed 32-bit integer value and store the result into a vector register.
///
/// `v_cvt_u16_f16`: Convert from a half-precision float input to an unsigned 16-bit integer value and store the result into a vector register.
///
/// `v_cvt_u32_f32`: Convert from a single-precision float input to an unsigned 32-bit integer value and store the result into a vector register.
///
/// `v_cvt_u32_f64`: Convert from a double-precision float input to an unsigned 32-bit integer value and store the result into a vector register.
pub type VCvt<D, S0, E> = crate::VectorUnary<family::VCvt, D, S0, E>;
/// `v_cvt_flr_i32_f32`: Convert from a single-precision float input to a signed 32-bit integer value using round-down semantics (ignore the default rounding mode) and store the result into a vector register.
pub type VCvtFlr<D, S0, E> = crate::VectorUnary<family::VCvtFlr, D, S0, E>;
/// `v_cvt_norm_i16_f16`: Convert from a half-precision float input to a signed normalized short and store the result into a vector register.
///
/// `v_cvt_norm_u16_f16`: Convert from a half-precision float input to an unsigned normalized short and store the result into a vector register.
pub type VCvtNorm<D, S0, E> = crate::VectorUnary<family::VCvtNorm, D, S0, E>;
/// `v_cvt_rpi_i32_f32`: Convert from a single-precision float input to a signed 32-bit integer value using round to nearest integer semantics (ignore the default rounding mode) and store the result into a vector register.
pub type VCvtRpi<D, S0, E> = crate::VectorUnary<family::VCvtRpi, D, S0, E>;
/// `v_div_fixup_f16`: Given a half-precision float quotient in the first input, a denominator in the second input and a numerator in the third input, detect and apply corner cases related to division, including divide by zero, NaN inputs and overflow, and modify the quotient accordingly. Generate any invalid, denormal and divide-by-zero exceptions that are a result of the division. Store the modified quotient into a vector register.
///
/// `v_div_fixup_f32`: Given a single-precision float quotient in the first input, a denominator in the second input and a numerator in the third input, detect and apply corner cases related to division, including divide by zero, NaN inputs and overflow, and modify the quotient accordingly. Generate any invalid, denormal and divide-by-zero exceptions that are a result of the division. Store the modified quotient into a vector register.
///
/// `v_div_fixup_f64`: Given a double-precision float quotient in the first input, a denominator in the second input and a numerator in the third input, detect and apply corner cases related to division, including divide by zero, NaN inputs and overflow, and modify the quotient accordingly. Generate any invalid, denormal and divide-by-zero exceptions that are a result of the division. Store the modified quotient into a vector register.
pub type VDivFixup<T, E> = crate::VectorTernary<family::VDivFixup, T, T, T, T, E>;
/// `v_exp_f16`: Calculate 2 raised to the power of the half-precision float input and store the result into a vector register.
///
/// `v_exp_f32`: Calculate 2 raised to the power of the single-precision float input and store the result into a vector register.
pub type VExp<T, E> = crate::VectorUnary<family::VExp, T, T, E>;
/// `v_ffbh_i32`: Count the number of leading bits that are the same as the sign bit of a vector input and store the result into a vector register. Store -1 if all input bits are the same.
///
/// `v_ffbh_u32`: Count the number of leading "0" bits before the first "1" in a vector input and store the result into a vector register. Store -1 if there are no "1" bits.
pub type VFfbh<D, S0, E> = crate::VectorUnary<family::VFfbh, D, S0, E>;
/// `v_ffbl_b32`: Count the number of trailing "0" bits before the first "1" in a vector input and store the result into a vector register. Store -1 if there are no "1" bits in the input.
pub type VFfbl<T, E> = crate::VectorUnary<family::VFfbl, T, T, E>;
/// `v_floor_f16`: Round the half-precision float input down to previous integer and store the result in floating point format into a vector register.
///
/// `v_floor_f32`: Round the single-precision float input down to previous integer and store the result in floating point format into a vector register.
///
/// `v_floor_f64`: Round the double-precision float input down to previous integer and store the result in floating point format into a vector register.
pub type VFloor<T, E> = crate::VectorUnary<family::VFloor, T, T, E>;
/// `v_fma_f16`: Multiply two half-precision float inputs and add a third input using fused multiply add, and store the result into a vector register.
///
/// `v_fma_f32`: Multiply two single-precision float inputs and add a third input using fused multiply add, and store the result into a vector register.
///
/// `v_fma_f64`: Multiply two double-precision float inputs and add a third input using fused multiply add, and store the result into a vector register.
pub type VFma<T, E> = crate::VectorTernary<family::VFma, T, T, T, T, E>;
/// `v_fma_legacy_f32`: Multiply and add single-precision values. Follows DX9 rules where 0.0 times anything produces 0.0.
pub type VFmaLegacy<T, E> = crate::VectorTernary<family::VFmaLegacy, T, T, T, T, E>;
/// `v_fmac_f16`: Multiply two half-precision float inputs and accumulate the result into the destination register using fused multiply add.
///
/// `v_fmac_f32`: Multiply two single-precision float inputs and accumulate the result into the destination register using fused multiply add.
pub type VFmac<T, E> = crate::VectorBinary<family::VFmac, T, T, T, E>;
/// `v_fmac_legacy_f32`: Multiply two single-precision values and accumulate the result with the destination. Follows DX9 rules where 0.0 times anything produces 0.0.
pub type VFmacLegacy<T, E> = crate::VectorBinary<family::VFmacLegacy, T, T, T, E>;
/// `v_fract_f16`: Compute the fractional portion of a half-precision float input and store the result in floating point format into a vector register.
///
/// `v_fract_f32`: Compute the fractional portion of a single-precision float input and store the result in floating point format into a vector register.
///
/// `v_fract_f64`: Compute the fractional portion of a double-precision float input and store the result in floating point format into a vector register.
pub type VFract<T, E> = crate::VectorUnary<family::VFract, T, T, E>;
/// `v_frexp_exp_i16_f16`: Extract the exponent of a half-precision float input and store the result as a signed 16-bit integer into a vector register.
///
/// `v_frexp_exp_i32_f32`: Extract the exponent of a single-precision float input and store the result as a signed 32-bit integer into a vector register.
///
/// `v_frexp_exp_i32_f64`: Extract the exponent of a double-precision float input and store the result as a signed 32-bit integer into a vector register.
pub type VFrexpExp<D, S0, E> = crate::VectorUnary<family::VFrexpExp, D, S0, E>;
/// `v_frexp_mant_f16`: Extract the binary significand, or mantissa, of a half-precision float input and store the result as a half-precision float into a vector register.
///
/// `v_frexp_mant_f32`: Extract the binary significand, or mantissa, of a single-precision float input and store the result as a single-precision float into a vector register.
///
/// `v_frexp_mant_f64`: Extract the binary significand, or mantissa, of a double-precision float input and store the result as a double-precision float into a vector register.
pub type VFrexpMant<T, E> = crate::VectorUnary<family::VFrexpMant, T, T, E>;
/// `v_ldexp_f16`: Multiply the first input, a floating point value, by an integral power of 2 specified in the second input, a signed integer value, and store the floating point result into a vector register.
///
/// `v_ldexp_f32`: Multiply the first input, a floating point value, by an integral power of 2 specified in the second input, a signed integer value, and store the floating point result into a vector register.
///
/// `v_ldexp_f64`: Multiply the first input, a floating point value, by an integral power of 2 specified in the second input, a signed integer value, and store the floating point result into a vector register.
pub type VLdexp<D, S0, S1, E> = crate::VectorBinary<family::VLdexp, D, S0, S1, E>;
/// `v_lerp_u8`: Average two 4-D vectors stored as packed bytes in the first two inputs with rounding control provided by the third input, then store the result into a vector register. Each byte in the third input acts as a rounding mode for the corresponding element; if the LSB is set then 0.5 rounds up, otherwise 0.5 truncates.
pub type VLerp<D, S0, S1, S2, E> = crate::VectorTernary<family::VLerp, D, S0, S1, S2, E>;
/// `v_log_f16`: Calculate the base 2 logarithm of the half-precision float input and store the result into a vector register.
///
/// `v_log_f32`: Calculate the base 2 logarithm of the single-precision float input and store the result into a vector register.
pub type VLog<T, E> = crate::VectorUnary<family::VLog, T, T, E>;
/// `v_lshl_add_u32`: Given a shift count in the second input, calculate the logical shift left of the first input, then add the third input to the intermediate result, then store the final result into a vector register.
pub type VLshlAdd<T, E> = crate::VectorTernary<family::VLshlAdd, T, T, T, T, E>;
/// `v_lshl_or_b32`: Given a shift count in the second input, calculate the logical shift left of the first input, then calculate the bitwise OR of the intermediate result and the third input, then store the final result into a vector register.
pub type VLshlOr<T, E> = crate::VectorTernary<family::VLshlOr, T, T, T, T, E>;
/// `v_lshlrev_b16`: Given a shift count in the first vector input, calculate the logical shift left of the second vector input and store the result into a vector register.
///
/// `v_lshlrev_b32`: Given a shift count in the first vector input, calculate the logical shift left of the second vector input and store the result into a vector register.
///
/// `v_lshlrev_b64`: Given a shift count in the first vector input, calculate the logical shift left of the second vector input and store the result into a vector register.
pub type VLshlrev<D, S0, S1, E> = crate::VectorBinary<family::VLshlrev, D, S0, S1, E>;
/// `v_lshrrev_b16`: Given a shift count in the first vector input, calculate the logical shift right of the second vector input and store the result into a vector register.
///
/// `v_lshrrev_b32`: Given a shift count in the first vector input, calculate the logical shift right of the second vector input and store the result into a vector register.
///
/// `v_lshrrev_b64`: Given a shift count in the first vector input, calculate the logical shift right of the second vector input and store the result into a vector register.
pub type VLshrrev<D, S0, S1, E> = crate::VectorBinary<family::VLshrrev, D, S0, S1, E>;
/// `v_mad_i16`: Multiply two signed 16-bit integer inputs, add a signed 16-bit integer value from a third input, and store the result into a vector register.
///
/// `v_mad_i32_i16`: Multiply two signed 16-bit integer inputs in the signed 32-bit integer domain, add a signed 32-bit integer value from a third input, and store the result as a signed 32-bit integer into a vector register.
///
/// `v_mad_u16`: Multiply two unsigned 16-bit integer inputs, add an unsigned 16-bit integer value from a third input, and store the result into a vector register.
///
/// `v_mad_u32_u16`: Multiply two unsigned 16-bit integer inputs in the unsigned 32-bit integer domain, add an unsigned 32-bit integer value from a third input, and store the result as an unsigned 32-bit integer into a vector register.
pub type VMad<D, S0, S1, S2, E> = crate::VectorTernary<family::VMad, D, S0, S1, S2, E>;
/// `v_max_f16`: Select the maximum of two half-precision float inputs and store the selected value into a vector register.
///
/// `v_max_f32`: Select the maximum of two single-precision float inputs and store the selected value into a vector register.
///
/// `v_max_f64`: Select the maximum of two double-precision float inputs and store the selected value into a vector register.
///
/// `v_max_i16`: Select the maximum of two signed 16-bit integer inputs and store the selected value into a vector register.
///
/// `v_max_i32`: Select the maximum of two signed 32-bit integer inputs and store the selected value into a vector register.
///
/// `v_max_u16`: Select the maximum of two unsigned 16-bit integer inputs and store the selected value into a vector register.
///
/// `v_max_u32`: Select the maximum of two unsigned 32-bit integer inputs and store the selected value into a vector register.
pub type VMax<T, E> = crate::VectorBinary<family::VMax, T, T, T, E>;
/// `v_max3_f16`: Select the maximum of three half-precision float inputs and store the selected value into a vector register.
///
/// `v_max3_f32`: Select the maximum of three single-precision float inputs and store the selected value into a vector register.
///
/// `v_max3_i16`: Select the maximum of three signed 16-bit integer inputs and store the selected value into a vector register.
///
/// `v_max3_i32`: Select the maximum of three signed 32-bit integer inputs and store the selected value into a vector register.
///
/// `v_max3_u16`: Select the maximum of three unsigned 16-bit integer inputs and store the selected value into a vector register.
///
/// `v_max3_u32`: Select the maximum of three unsigned 32-bit integer inputs and store the selected value into a vector register.
pub type VMax3<T, E> = crate::VectorTernary<family::VMax3, T, T, T, T, E>;
/// `v_mbcnt_hi_u32_b32`: For each lane 32 <= N < 64, examine the N least significant bits of the first input and count how many of those bits are "1". For lane positions 0 <= N < 32 no bits are examined and the count is zero. Add this count to the value in the second input and store the result into a vector register.
pub type VMbcntHi<T, E> = crate::VectorBinary<family::VMbcntHi, T, T, T, E>;
/// `v_mbcnt_lo_u32_b32`: For each lane 0 <= N < 32, examine the N least significant bits of the first input and count how many of those bits are "1". For each lane 32 <= N < 64, all "1" bits in the first input are counted. Add this count to the value in the second input and store the result into a vector register.
pub type VMbcntLo<T, E> = crate::VectorBinary<family::VMbcntLo, T, T, T, E>;
/// `v_med3_f16`: Select the median of three half-precision float inputs and store the selected value into a vector register.
///
/// `v_med3_f32`: Select the median of three single-precision float inputs and store the selected value into a vector register.
///
/// `v_med3_i16`: Select the median of three signed 16-bit integer inputs and store the selected value into a vector register.
///
/// `v_med3_i32`: Select the median of three signed 32-bit integer inputs and store the selected value into a vector register.
///
/// `v_med3_u16`: Select the median of three unsigned 16-bit integer inputs and store the selected value into a vector register.
///
/// `v_med3_u32`: Select the median of three unsigned 32-bit integer inputs and store the selected value into a vector register.
pub type VMed3<T, E> = crate::VectorTernary<family::VMed3, T, T, T, T, E>;
/// `v_min_f16`: Select the minimum of two half-precision float inputs and store the selected value into a vector register.
///
/// `v_min_f32`: Select the minimum of two single-precision float inputs and store the selected value into a vector register.
///
/// `v_min_f64`: Select the minimum of two double-precision float inputs and store the selected value into a vector register.
///
/// `v_min_i16`: Select the minimum of two signed 16-bit integer inputs and store the selected value into a vector register.
///
/// `v_min_i32`: Select the minimum of two signed 32-bit integer inputs and store the selected value into a vector register.
///
/// `v_min_u16`: Select the minimum of two unsigned 16-bit integer inputs and store the selected value into a vector register.
///
/// `v_min_u32`: Select the minimum of two unsigned 32-bit integer inputs and store the selected value into a vector register.
pub type VMin<T, E> = crate::VectorBinary<family::VMin, T, T, T, E>;
/// `v_min3_f16`: Select the minimum of three half-precision float inputs and store the selected value into a vector register.
///
/// `v_min3_f32`: Select the minimum of three single-precision float inputs and store the selected value into a vector register.
///
/// `v_min3_i16`: Select the minimum of three signed 16-bit integer inputs and store the selected value into a vector register.
///
/// `v_min3_i32`: Select the minimum of three signed 32-bit integer inputs and store the selected value into a vector register.
///
/// `v_min3_u16`: Select the minimum of three unsigned 16-bit integer inputs and store the selected value into a vector register.
///
/// `v_min3_u32`: Select the minimum of three unsigned 32-bit integer inputs and store the selected value into a vector register.
pub type VMin3<T, E> = crate::VectorTernary<family::VMin3, T, T, T, T, E>;
/// `v_mov_b32`: Move 32-bit data from a vector input into a vector register.
pub type VMov<T, E> = crate::VectorUnary<family::VMov, T, T, E>;
/// `v_mqsad_pk_u16_u8`: Perform the V_MSAD_U8 operation four times using different slices of the first array, all entries of the second array and each entry of the third array. Truncate each result to 16 bits, pack the values into a 4-entry array and store the array into a vector register. The first input is an 8-entry array of unsigned 8-bit integers, the second input is a 4-entry array of unsigned 8-bit integers and the third input is a 4-entry array of unsigned 16-bit integers.
pub type VMqsadPk<D, S0, S1, S2, E> = crate::VectorTernary<family::VMqsadPk, D, S0, S1, S2, E>;
/// `v_msad_u8`: Calculate the sum of absolute differences of elements in two packed 4-component unsigned 8-bit integer inputs, except that elements where the second input (known as the reference input) is zero are not included in the sum. Add an unsigned 32-bit integer value from the third input and store the result into a vector register.
pub type VMsad<D, S0, S1, S2, E> = crate::VectorTernary<family::VMsad, D, S0, S1, S2, E>;
/// `v_mul_f16`: Multiply two floating point inputs and store the result into a vector register.
///
/// `v_mul_f32`: Multiply two floating point inputs and store the result into a vector register.
///
/// `v_mul_f64`: Multiply two floating point inputs and store the result into a vector register.
pub type VMul<T, E> = crate::VectorBinary<family::VMul, T, T, T, E>;
/// `v_mul_hi_i32`: Multiply two signed 32-bit integer inputs and store the high 32 bits of the result into a vector register.
///
/// `v_mul_hi_u32`: Multiply two unsigned 32-bit integer inputs and store the high 32 bits of the result into a vector register.
pub type VMulHi<T, E> = crate::VectorBinary<family::VMulHi, T, T, T, E>;
/// `v_mul_legacy_f32`: Multiply two floating point inputs and store the result into a vector register. Follows DX9 rules where 0.0 times anything produces 0.0 (this differs from other APIs when the other input is infinity or NaN).
pub type VMulLegacy<T, E> = crate::VectorBinary<family::VMulLegacy, T, T, T, E>;
/// `v_mul_lo_u16`: Multiply two unsigned 16-bit integer inputs and store the low bits of the result into a vector register.
///
/// `v_mul_lo_u32`: Multiply two unsigned 32-bit integer inputs and store the result into a vector register.
pub type VMulLo<T, E> = crate::VectorBinary<family::VMulLo, T, T, T, E>;
/// `v_mullit_f32`: Multiply two floating point inputs and store the result into a vector register. Specific rules apply to accommodate lighting calculations: 0.0 * x = 0.0 and alternate INF, NAN, overflow rules apply.
pub type VMullit<T, E> = crate::VectorTernary<family::VMullit, T, T, T, T, E>;
/// `v_not_b32`: Calculate bitwise negation on a vector input and store the result into a vector register.
pub type VNot<T, E> = crate::VectorUnary<family::VNot, T, T, E>;
/// `v_or_b32`: Calculate bitwise OR on two vector inputs and store the result into a vector register.
pub type VOr<T, E> = crate::VectorBinary<family::VOr, T, T, T, E>;
/// `v_or3_b32`: Calculate the bitwise OR of three vector inputs and store the result into a vector register.
pub type VOr3<T, E> = crate::VectorTernary<family::VOr3, T, T, T, T, E>;
/// `v_pack_b32_f16`: Pack two half-precision float values into a single 32-bit value and store the result into a vector register.
pub type VPack<D, S0, S1, E> = crate::VectorBinary<family::VPack, D, S0, S1, E>;
/// `v_qsad_pk_u16_u8`: Perform the V_SAD_U8 operation four times using different slices of the first array, all entries of the second array and each entry of the third array. Truncate each result to 16 bits, pack the values into a 4-entry array and store the array into a vector register. The first input is an 8-entry array of unsigned 8-bit integers, the second input is a 4-entry array of unsigned 8-bit integers and the third input is a 4-entry array of unsigned 16-bit integers.
pub type VQsadPk<D, S0, S1, S2, E> = crate::VectorTernary<family::VQsadPk, D, S0, S1, S2, E>;
/// `v_rcp_f16`: Calculate the reciprocal of the half-precision float input using IEEE rules and store the result into a vector register.
///
/// `v_rcp_f32`: Calculate the reciprocal of the single-precision float input using IEEE rules and store the result into a vector register.
///
/// `v_rcp_f64`: Calculate the reciprocal of the double-precision float input using IEEE rules and store the result into a vector register.
pub type VRcp<T, E> = crate::VectorUnary<family::VRcp, T, T, E>;
/// `v_rcp_iflag_f32`: Calculate the reciprocal of the vector float input in a manner suitable for integer division and store the result into a vector register. This opcode is intended for use as part of an integer division macro.
pub type VRcpIflag<T, E> = crate::VectorUnary<family::VRcpIflag, T, T, E>;
/// `v_rndne_f16`: Round the half-precision float input to the nearest even integer and store the result in floating point format into a vector register.
///
/// `v_rndne_f32`: Round the single-precision float input to the nearest even integer and store the result in floating point format into a vector register.
///
/// `v_rndne_f64`: Round the double-precision float input to the nearest even integer and store the result in floating point format into a vector register.
pub type VRndne<T, E> = crate::VectorUnary<family::VRndne, T, T, E>;
/// `v_rsq_f16`: Calculate the reciprocal of the square root of the half-precision float input using IEEE rules and store the result into a vector register.
///
/// `v_rsq_f32`: Calculate the reciprocal of the square root of the single-precision float input using IEEE rules and store the result into a vector register.
///
/// `v_rsq_f64`: Calculate the reciprocal of the square root of the double-precision float input using IEEE rules and store the result into a vector register.
pub type VRsq<T, E> = crate::VectorUnary<family::VRsq, T, T, E>;
/// `v_sad_u32`: Calculate the absolute difference of two unsigned 32-bit integer inputs, add an unsigned 32-bit integer value from the third input and store the result into a vector register.
pub type VSad<T, E> = crate::VectorTernary<family::VSad, T, T, T, T, E>;
/// `v_sin_f16`: Calculate the trigonometric sine of a half-precision float value using IEEE rules and store the result into a vector register. The operand is calculated by scaling the vector input by 2 PI.
///
/// `v_sin_f32`: Calculate the trigonometric sine of a single-precision float value using IEEE rules and store the result into a vector register. The operand is calculated by scaling the vector input by 2 PI.
pub type VSin<T, E> = crate::VectorUnary<family::VSin, T, T, E>;
/// `v_sqrt_f16`: Calculate the square root of the half-precision float input using IEEE rules and store the result into a vector register.
///
/// `v_sqrt_f32`: Calculate the square root of the single-precision float input using IEEE rules and store the result into a vector register.
///
/// `v_sqrt_f64`: Calculate the square root of the double-precision float input using IEEE rules and store the result into a vector register.
pub type VSqrt<T, E> = crate::VectorUnary<family::VSqrt, T, T, E>;
/// `v_sub_f16`: Subtract the second floating point input from the first input and store the result into a vector register.
///
/// `v_sub_f32`: Subtract the second floating point input from the first input and store the result into a vector register.
pub type VSub<T, E> = crate::VectorBinary<family::VSub, T, T, T, E>;
/// `v_sub_nc_i16`: Subtract the second signed 16-bit integer input from the first input and store the result into a vector register. No carry-in or carry-out support.
///
/// `v_sub_nc_i32`: Subtract the second signed 32-bit integer input from the first input and store the result into a vector register. No carry-in or carry-out support.
///
/// `v_sub_nc_u16`: Subtract the second unsigned 16-bit integer input from the first input and store the result into a vector register. No carry-in or carry-out support.
///
/// `v_sub_nc_u32`: Subtract the second unsigned 32-bit integer input from the first input and store the result into a vector register. No carry-in or carry-out support.
pub type VSubNc<T, E> = crate::VectorBinary<family::VSubNc, T, T, T, E>;
/// `v_subrev_f16`: Subtract the first floating point input from the second input and store the result into a vector register.
///
/// `v_subrev_f32`: Subtract the first floating point input from the second input and store the result into a vector register.
pub type VSubrev<T, E> = crate::VectorBinary<family::VSubrev, T, T, T, E>;
/// `v_subrev_nc_u32`: Subtract the first unsigned 32-bit integer input from the second input and store the result into a vector register. No carry-in or carry-out support.
pub type VSubrevNc<T, E> = crate::VectorBinary<family::VSubrevNc, T, T, T, E>;
/// `v_trig_preop_f64`: Look up a 53-bit segment of 2/PI using an integer segment select in the second input. Scale the intermediate result by the exponent from the first double-precision float input and store the double-precision float result into a vector register.
pub type VTrigPreop<D, S0, S1, E> = crate::VectorBinary<family::VTrigPreop, D, S0, S1, E>;
/// `v_trunc_f16`: Compute the integer part of a half-precision float input using round toward zero semantics and store the result in floating point format into a vector register.
///
/// `v_trunc_f32`: Compute the integer part of a single-precision float input using round toward zero semantics and store the result in floating point format into a vector register.
///
/// `v_trunc_f64`: Compute the integer part of a double-precision float input using round toward zero semantics and store the result in floating point format into a vector register.
pub type VTrunc<T, E> = crate::VectorUnary<family::VTrunc, T, T, E>;
/// `v_xad_u32`: Calculate bitwise XOR of the first two vector inputs, then add the third vector input to the intermediate result, then store the final result into a vector register.
pub type VXad<T, E> = crate::VectorTernary<family::VXad, T, T, T, T, E>;
/// `v_xnor_b32`: Calculate bitwise XNOR on two vector inputs and store the result into a vector register.
pub type VXnor<T, E> = crate::VectorBinary<family::VXnor, T, T, T, E>;
/// `v_xor_b32`: Calculate bitwise XOR on two vector inputs and store the result into a vector register.
pub type VXor<T, E> = crate::VectorBinary<family::VXor, T, T, T, E>;
/// `v_xor3_b32`: Calculate the bitwise XOR of three vector inputs and store the result into a vector register.
pub type VXor3<T, E> = crate::VectorTernary<family::VXor3, T, T, T, T, E>;
impl
    crate::private::Ternary<
        family::VAdd3,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VAdd3,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::U32>;
    type Source1 = crate::ModifiedSource<crate::U32>;
    type Source2 = crate::ModifiedSource<crate::U32>;
}
impl crate::private::Binary<family::VAdd, crate::F16, crate::F16, crate::F16, crate::E32> for () {}
impl crate::SupportedBinary<family::VAdd, crate::F16, crate::F16, crate::F16, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::F16>;
    type Source1 = crate::VectorRegister<crate::F16>;
}
impl crate::private::Binary<family::VAdd, crate::F16, crate::F16, crate::F16, crate::E64> for () {}
impl crate::SupportedBinary<family::VAdd, crate::F16, crate::F16, crate::F16, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::F16>;
    type Source1 = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Binary<family::VAdd, crate::F32, crate::F32, crate::F32, crate::E32> for () {}
impl crate::SupportedBinary<family::VAdd, crate::F32, crate::F32, crate::F32, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::F32>;
    type Source1 = crate::VectorRegister<crate::F32>;
}
impl crate::private::Binary<family::VAdd, crate::F32, crate::F32, crate::F32, crate::E64> for () {}
impl crate::SupportedBinary<family::VAdd, crate::F32, crate::F32, crate::F32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::F32>;
    type Source1 = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Binary<family::VAdd, crate::F64, crate::F64, crate::F64, crate::E64> for () {}
impl crate::SupportedBinary<family::VAdd, crate::F64, crate::F64, crate::F64, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::F64>;
    type Source1 = crate::ModifiedSource<crate::F64>;
}
impl
    crate::private::Ternary<
        family::VAddLshl,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VAddLshl,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::U32>;
    type Source1 = crate::ModifiedSource<crate::U32>;
    type Source2 = crate::ModifiedSource<crate::U32>;
}
impl crate::private::Binary<family::VAddNc, crate::I16, crate::I16, crate::I16, crate::E64> for () {}
impl crate::SupportedBinary<family::VAddNc, crate::I16, crate::I16, crate::I16, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::I16>;
    type Source1 = crate::ModifiedSource<crate::I16>;
}
impl crate::private::Binary<family::VAddNc, crate::I32, crate::I32, crate::I32, crate::E64> for () {}
impl crate::SupportedBinary<family::VAddNc, crate::I32, crate::I32, crate::I32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::I32>;
    type Source1 = crate::ModifiedSource<crate::I32>;
}
impl crate::private::Binary<family::VAddNc, crate::U16, crate::U16, crate::U16, crate::E64> for () {}
impl crate::SupportedBinary<family::VAddNc, crate::U16, crate::U16, crate::U16, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::U16>;
    type Source1 = crate::ModifiedSource<crate::U16>;
}
impl crate::private::Binary<family::VAddNc, crate::U32, crate::U32, crate::U32, crate::E32> for () {}
impl crate::SupportedBinary<family::VAddNc, crate::U32, crate::U32, crate::U32, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::U32>;
    type Source1 = crate::VectorRegister<crate::U32>;
}
impl crate::private::Binary<family::VAddNc, crate::U32, crate::U32, crate::U32, crate::E64> for () {}
impl crate::SupportedBinary<family::VAddNc, crate::U32, crate::U32, crate::U32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::U32>;
    type Source1 = crate::ModifiedSource<crate::U32>;
}
impl
    crate::private::Ternary<
        family::VAlignbit,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::U8,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VAlignbit,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::U8,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::B32>;
    type Source1 = crate::ModifiedSource<crate::B32>;
    type Source2 = crate::ModifiedSource<crate::U8>;
}
impl
    crate::private::Ternary<
        family::VAlignbyte,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::U8,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VAlignbyte,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::U8,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::B32>;
    type Source1 = crate::ModifiedSource<crate::B32>;
    type Source2 = crate::ModifiedSource<crate::U8>;
}
impl crate::private::Binary<family::VAnd, crate::B32, crate::B32, crate::B32, crate::E32> for () {}
impl crate::SupportedBinary<family::VAnd, crate::B32, crate::B32, crate::B32, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::B32>;
    type Source1 = crate::VectorRegister<crate::B32>;
}
impl crate::private::Binary<family::VAnd, crate::B32, crate::B32, crate::B32, crate::E64> for () {}
impl crate::SupportedBinary<family::VAnd, crate::B32, crate::B32, crate::B32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::B32>;
    type Source1 = crate::ModifiedSource<crate::B32>;
}
impl
    crate::private::Ternary<
        family::VAndOr,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VAndOr,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::B32>;
    type Source1 = crate::ModifiedSource<crate::B32>;
    type Source2 = crate::ModifiedSource<crate::B32>;
}
impl crate::private::Binary<family::VAshrrev, crate::I16, crate::U16, crate::I16, crate::E64>
    for ()
{
}
impl crate::SupportedBinary<family::VAshrrev, crate::I16, crate::U16, crate::I16, crate::E64>
    for ()
{
    type Source0 = crate::ModifiedSource<crate::U16>;
    type Source1 = crate::ModifiedSource<crate::I16>;
}
impl crate::private::Binary<family::VAshrrev, crate::I32, crate::U32, crate::I32, crate::E32>
    for ()
{
}
impl crate::SupportedBinary<family::VAshrrev, crate::I32, crate::U32, crate::I32, crate::E32>
    for ()
{
    type Source0 = crate::SourceOperand<crate::U32>;
    type Source1 = crate::VectorRegister<crate::I32>;
}
impl crate::private::Binary<family::VAshrrev, crate::I32, crate::U32, crate::I32, crate::E64>
    for ()
{
}
impl crate::SupportedBinary<family::VAshrrev, crate::I32, crate::U32, crate::I32, crate::E64>
    for ()
{
    type Source0 = crate::ModifiedSource<crate::U32>;
    type Source1 = crate::ModifiedSource<crate::I32>;
}
impl crate::private::Binary<family::VAshrrev, crate::I64, crate::U32, crate::I64, crate::E64>
    for ()
{
}
impl crate::SupportedBinary<family::VAshrrev, crate::I64, crate::U32, crate::I64, crate::E64>
    for ()
{
    type Source0 = crate::ModifiedSource<crate::U32>;
    type Source1 = crate::ModifiedSource<crate::I64>;
}
impl crate::private::Binary<family::VBcnt, crate::B32, crate::B32, crate::B32, crate::E64> for () {}
impl crate::SupportedBinary<family::VBcnt, crate::B32, crate::B32, crate::B32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::B32>;
    type Source1 = crate::ModifiedSource<crate::B32>;
}
impl
    crate::private::Ternary<
        family::VBfe,
        crate::I32,
        crate::I32,
        crate::U32,
        crate::U32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VBfe,
        crate::I32,
        crate::I32,
        crate::U32,
        crate::U32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::I32>;
    type Source1 = crate::ModifiedSource<crate::U32>;
    type Source2 = crate::ModifiedSource<crate::U32>;
}
impl
    crate::private::Ternary<
        family::VBfe,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VBfe,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::U32>;
    type Source1 = crate::ModifiedSource<crate::U32>;
    type Source2 = crate::ModifiedSource<crate::U32>;
}
impl
    crate::private::Ternary<
        family::VBfi,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VBfi,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::B32>;
    type Source1 = crate::ModifiedSource<crate::B32>;
    type Source2 = crate::ModifiedSource<crate::B32>;
}
impl crate::private::Binary<family::VBfm, crate::B32, crate::B32, crate::B32, crate::E64> for () {}
impl crate::SupportedBinary<family::VBfm, crate::B32, crate::B32, crate::B32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::B32>;
    type Source1 = crate::ModifiedSource<crate::B32>;
}
impl crate::private::Unary<family::VBfrev, crate::B32, crate::B32, crate::E32> for () {}
impl crate::SupportedUnary<family::VBfrev, crate::B32, crate::B32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::B32>;
}
impl crate::private::Unary<family::VBfrev, crate::B32, crate::B32, crate::E64> for () {}
impl crate::SupportedUnary<family::VBfrev, crate::B32, crate::B32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::B32>;
}
impl crate::private::Unary<family::VCeil, crate::F16, crate::F16, crate::E32> for () {}
impl crate::SupportedUnary<family::VCeil, crate::F16, crate::F16, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F16>;
}
impl crate::private::Unary<family::VCeil, crate::F16, crate::F16, crate::E64> for () {}
impl crate::SupportedUnary<family::VCeil, crate::F16, crate::F16, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Unary<family::VCeil, crate::F32, crate::F32, crate::E32> for () {}
impl crate::SupportedUnary<family::VCeil, crate::F32, crate::F32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F32>;
}
impl crate::private::Unary<family::VCeil, crate::F32, crate::F32, crate::E64> for () {}
impl crate::SupportedUnary<family::VCeil, crate::F32, crate::F32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Unary<family::VCeil, crate::F64, crate::F64, crate::E32> for () {}
impl crate::SupportedUnary<family::VCeil, crate::F64, crate::F64, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F64>;
}
impl crate::private::Unary<family::VCeil, crate::F64, crate::F64, crate::E64> for () {}
impl crate::SupportedUnary<family::VCeil, crate::F64, crate::F64, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F64>;
}
impl crate::private::Unary<family::VCos, crate::F16, crate::F16, crate::E32> for () {}
impl crate::SupportedUnary<family::VCos, crate::F16, crate::F16, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F16>;
}
impl crate::private::Unary<family::VCos, crate::F16, crate::F16, crate::E64> for () {}
impl crate::SupportedUnary<family::VCos, crate::F16, crate::F16, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Unary<family::VCos, crate::F32, crate::F32, crate::E32> for () {}
impl crate::SupportedUnary<family::VCos, crate::F32, crate::F32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F32>;
}
impl crate::private::Unary<family::VCos, crate::F32, crate::F32, crate::E64> for () {}
impl crate::SupportedUnary<family::VCos, crate::F32, crate::F32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F32>;
}
impl
    crate::private::Ternary<
        family::VCubeid,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VCubeid,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::F32>;
    type Source1 = crate::ModifiedSource<crate::F32>;
    type Source2 = crate::ModifiedSource<crate::F32>;
}
impl
    crate::private::Ternary<
        family::VCubema,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VCubema,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::F32>;
    type Source1 = crate::ModifiedSource<crate::F32>;
    type Source2 = crate::ModifiedSource<crate::F32>;
}
impl
    crate::private::Ternary<
        family::VCubesc,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VCubesc,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::F32>;
    type Source1 = crate::ModifiedSource<crate::F32>;
    type Source2 = crate::ModifiedSource<crate::F32>;
}
impl
    crate::private::Ternary<
        family::VCubetc,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VCubetc,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::F32>;
    type Source1 = crate::ModifiedSource<crate::F32>;
    type Source2 = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Unary<family::VCvt, crate::F16, crate::F32, crate::E32> for () {}
impl crate::SupportedUnary<family::VCvt, crate::F16, crate::F32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F32>;
}
impl crate::private::Unary<family::VCvt, crate::F16, crate::F32, crate::E64> for () {}
impl crate::SupportedUnary<family::VCvt, crate::F16, crate::F32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Unary<family::VCvt, crate::F16, crate::I16, crate::E32> for () {}
impl crate::SupportedUnary<family::VCvt, crate::F16, crate::I16, crate::E32> for () {
    type Source = crate::SourceOperand<crate::I16>;
}
impl crate::private::Unary<family::VCvt, crate::F16, crate::I16, crate::E64> for () {}
impl crate::SupportedUnary<family::VCvt, crate::F16, crate::I16, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::I16>;
}
impl crate::private::Unary<family::VCvt, crate::F16, crate::U16, crate::E32> for () {}
impl crate::SupportedUnary<family::VCvt, crate::F16, crate::U16, crate::E32> for () {
    type Source = crate::SourceOperand<crate::U16>;
}
impl crate::private::Unary<family::VCvt, crate::F16, crate::U16, crate::E64> for () {}
impl crate::SupportedUnary<family::VCvt, crate::F16, crate::U16, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::U16>;
}
impl crate::private::Unary<family::VCvt, crate::F32, crate::F16, crate::E32> for () {}
impl crate::SupportedUnary<family::VCvt, crate::F32, crate::F16, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F16>;
}
impl crate::private::Unary<family::VCvt, crate::F32, crate::F16, crate::E64> for () {}
impl crate::SupportedUnary<family::VCvt, crate::F32, crate::F16, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Unary<family::VCvt, crate::F32, crate::F64, crate::E32> for () {}
impl crate::SupportedUnary<family::VCvt, crate::F32, crate::F64, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F64>;
}
impl crate::private::Unary<family::VCvt, crate::F32, crate::F64, crate::E64> for () {}
impl crate::SupportedUnary<family::VCvt, crate::F32, crate::F64, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F64>;
}
impl crate::private::Unary<family::VCvt, crate::F32, crate::I32, crate::E32> for () {}
impl crate::SupportedUnary<family::VCvt, crate::F32, crate::I32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::I32>;
}
impl crate::private::Unary<family::VCvt, crate::F32, crate::I32, crate::E64> for () {}
impl crate::SupportedUnary<family::VCvt, crate::F32, crate::I32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::I32>;
}
impl crate::private::Unary<family::VCvt, crate::F32, crate::U32, crate::E32> for () {}
impl crate::SupportedUnary<family::VCvt, crate::F32, crate::U32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::U32>;
}
impl crate::private::Unary<family::VCvt, crate::F32, crate::U32, crate::E64> for () {}
impl crate::SupportedUnary<family::VCvt, crate::F32, crate::U32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::U32>;
}
impl crate::private::Unary<family::VCvt, crate::F64, crate::F32, crate::E32> for () {}
impl crate::SupportedUnary<family::VCvt, crate::F64, crate::F32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F32>;
}
impl crate::private::Unary<family::VCvt, crate::F64, crate::F32, crate::E64> for () {}
impl crate::SupportedUnary<family::VCvt, crate::F64, crate::F32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Unary<family::VCvt, crate::F64, crate::I32, crate::E32> for () {}
impl crate::SupportedUnary<family::VCvt, crate::F64, crate::I32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::I32>;
}
impl crate::private::Unary<family::VCvt, crate::F64, crate::I32, crate::E64> for () {}
impl crate::SupportedUnary<family::VCvt, crate::F64, crate::I32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::I32>;
}
impl crate::private::Unary<family::VCvt, crate::F64, crate::U32, crate::E32> for () {}
impl crate::SupportedUnary<family::VCvt, crate::F64, crate::U32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::U32>;
}
impl crate::private::Unary<family::VCvt, crate::F64, crate::U32, crate::E64> for () {}
impl crate::SupportedUnary<family::VCvt, crate::F64, crate::U32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::U32>;
}
impl crate::private::Unary<family::VCvt, crate::I16, crate::F16, crate::E32> for () {}
impl crate::SupportedUnary<family::VCvt, crate::I16, crate::F16, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F16>;
}
impl crate::private::Unary<family::VCvt, crate::I16, crate::F16, crate::E64> for () {}
impl crate::SupportedUnary<family::VCvt, crate::I16, crate::F16, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Unary<family::VCvt, crate::I32, crate::F32, crate::E32> for () {}
impl crate::SupportedUnary<family::VCvt, crate::I32, crate::F32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F32>;
}
impl crate::private::Unary<family::VCvt, crate::I32, crate::F32, crate::E64> for () {}
impl crate::SupportedUnary<family::VCvt, crate::I32, crate::F32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Unary<family::VCvt, crate::I32, crate::F64, crate::E32> for () {}
impl crate::SupportedUnary<family::VCvt, crate::I32, crate::F64, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F64>;
}
impl crate::private::Unary<family::VCvt, crate::I32, crate::F64, crate::E64> for () {}
impl crate::SupportedUnary<family::VCvt, crate::I32, crate::F64, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F64>;
}
impl crate::private::Unary<family::VCvt, crate::U16, crate::F16, crate::E32> for () {}
impl crate::SupportedUnary<family::VCvt, crate::U16, crate::F16, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F16>;
}
impl crate::private::Unary<family::VCvt, crate::U16, crate::F16, crate::E64> for () {}
impl crate::SupportedUnary<family::VCvt, crate::U16, crate::F16, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Unary<family::VCvt, crate::U32, crate::F32, crate::E32> for () {}
impl crate::SupportedUnary<family::VCvt, crate::U32, crate::F32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F32>;
}
impl crate::private::Unary<family::VCvt, crate::U32, crate::F32, crate::E64> for () {}
impl crate::SupportedUnary<family::VCvt, crate::U32, crate::F32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Unary<family::VCvt, crate::U32, crate::F64, crate::E32> for () {}
impl crate::SupportedUnary<family::VCvt, crate::U32, crate::F64, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F64>;
}
impl crate::private::Unary<family::VCvt, crate::U32, crate::F64, crate::E64> for () {}
impl crate::SupportedUnary<family::VCvt, crate::U32, crate::F64, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F64>;
}
impl crate::private::Unary<family::VCvtFlr, crate::I32, crate::F32, crate::E32> for () {}
impl crate::SupportedUnary<family::VCvtFlr, crate::I32, crate::F32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F32>;
}
impl crate::private::Unary<family::VCvtFlr, crate::I32, crate::F32, crate::E64> for () {}
impl crate::SupportedUnary<family::VCvtFlr, crate::I32, crate::F32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Unary<family::VCvtNorm, crate::I16, crate::F16, crate::E32> for () {}
impl crate::SupportedUnary<family::VCvtNorm, crate::I16, crate::F16, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F16>;
}
impl crate::private::Unary<family::VCvtNorm, crate::I16, crate::F16, crate::E64> for () {}
impl crate::SupportedUnary<family::VCvtNorm, crate::I16, crate::F16, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Unary<family::VCvtNorm, crate::U16, crate::F16, crate::E32> for () {}
impl crate::SupportedUnary<family::VCvtNorm, crate::U16, crate::F16, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F16>;
}
impl crate::private::Unary<family::VCvtNorm, crate::U16, crate::F16, crate::E64> for () {}
impl crate::SupportedUnary<family::VCvtNorm, crate::U16, crate::F16, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Unary<family::VCvtRpi, crate::I32, crate::F32, crate::E32> for () {}
impl crate::SupportedUnary<family::VCvtRpi, crate::I32, crate::F32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F32>;
}
impl crate::private::Unary<family::VCvtRpi, crate::I32, crate::F32, crate::E64> for () {}
impl crate::SupportedUnary<family::VCvtRpi, crate::I32, crate::F32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F32>;
}
impl
    crate::private::Ternary<
        family::VDivFixup,
        crate::F16,
        crate::F16,
        crate::F16,
        crate::F16,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VDivFixup,
        crate::F16,
        crate::F16,
        crate::F16,
        crate::F16,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::F16>;
    type Source1 = crate::ModifiedSource<crate::F16>;
    type Source2 = crate::ModifiedSource<crate::F16>;
}
impl
    crate::private::Ternary<
        family::VDivFixup,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VDivFixup,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::F32>;
    type Source1 = crate::ModifiedSource<crate::F32>;
    type Source2 = crate::ModifiedSource<crate::F32>;
}
impl
    crate::private::Ternary<
        family::VDivFixup,
        crate::F64,
        crate::F64,
        crate::F64,
        crate::F64,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VDivFixup,
        crate::F64,
        crate::F64,
        crate::F64,
        crate::F64,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::F64>;
    type Source1 = crate::ModifiedSource<crate::F64>;
    type Source2 = crate::ModifiedSource<crate::F64>;
}
impl crate::private::Unary<family::VExp, crate::F16, crate::F16, crate::E32> for () {}
impl crate::SupportedUnary<family::VExp, crate::F16, crate::F16, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F16>;
}
impl crate::private::Unary<family::VExp, crate::F16, crate::F16, crate::E64> for () {}
impl crate::SupportedUnary<family::VExp, crate::F16, crate::F16, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Unary<family::VExp, crate::F32, crate::F32, crate::E32> for () {}
impl crate::SupportedUnary<family::VExp, crate::F32, crate::F32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F32>;
}
impl crate::private::Unary<family::VExp, crate::F32, crate::F32, crate::E64> for () {}
impl crate::SupportedUnary<family::VExp, crate::F32, crate::F32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Unary<family::VFfbh, crate::I32, crate::I32, crate::E32> for () {}
impl crate::SupportedUnary<family::VFfbh, crate::I32, crate::I32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::I32>;
}
impl crate::private::Unary<family::VFfbh, crate::I32, crate::I32, crate::E64> for () {}
impl crate::SupportedUnary<family::VFfbh, crate::I32, crate::I32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::I32>;
}
impl crate::private::Unary<family::VFfbh, crate::I32, crate::U32, crate::E32> for () {}
impl crate::SupportedUnary<family::VFfbh, crate::I32, crate::U32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::U32>;
}
impl crate::private::Unary<family::VFfbh, crate::I32, crate::U32, crate::E64> for () {}
impl crate::SupportedUnary<family::VFfbh, crate::I32, crate::U32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::U32>;
}
impl crate::private::Unary<family::VFfbl, crate::B32, crate::B32, crate::E32> for () {}
impl crate::SupportedUnary<family::VFfbl, crate::B32, crate::B32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::B32>;
}
impl crate::private::Unary<family::VFfbl, crate::B32, crate::B32, crate::E64> for () {}
impl crate::SupportedUnary<family::VFfbl, crate::B32, crate::B32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::B32>;
}
impl crate::private::Unary<family::VFloor, crate::F16, crate::F16, crate::E32> for () {}
impl crate::SupportedUnary<family::VFloor, crate::F16, crate::F16, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F16>;
}
impl crate::private::Unary<family::VFloor, crate::F16, crate::F16, crate::E64> for () {}
impl crate::SupportedUnary<family::VFloor, crate::F16, crate::F16, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Unary<family::VFloor, crate::F32, crate::F32, crate::E32> for () {}
impl crate::SupportedUnary<family::VFloor, crate::F32, crate::F32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F32>;
}
impl crate::private::Unary<family::VFloor, crate::F32, crate::F32, crate::E64> for () {}
impl crate::SupportedUnary<family::VFloor, crate::F32, crate::F32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Unary<family::VFloor, crate::F64, crate::F64, crate::E32> for () {}
impl crate::SupportedUnary<family::VFloor, crate::F64, crate::F64, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F64>;
}
impl crate::private::Unary<family::VFloor, crate::F64, crate::F64, crate::E64> for () {}
impl crate::SupportedUnary<family::VFloor, crate::F64, crate::F64, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F64>;
}
impl
    crate::private::Ternary<
        family::VFma,
        crate::F16,
        crate::F16,
        crate::F16,
        crate::F16,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VFma,
        crate::F16,
        crate::F16,
        crate::F16,
        crate::F16,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::F16>;
    type Source1 = crate::ModifiedSource<crate::F16>;
    type Source2 = crate::ModifiedSource<crate::F16>;
}
impl
    crate::private::Ternary<
        family::VFma,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VFma,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::F32>;
    type Source1 = crate::ModifiedSource<crate::F32>;
    type Source2 = crate::ModifiedSource<crate::F32>;
}
impl
    crate::private::Ternary<
        family::VFma,
        crate::F64,
        crate::F64,
        crate::F64,
        crate::F64,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VFma,
        crate::F64,
        crate::F64,
        crate::F64,
        crate::F64,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::F64>;
    type Source1 = crate::ModifiedSource<crate::F64>;
    type Source2 = crate::ModifiedSource<crate::F64>;
}
impl
    crate::private::Ternary<
        family::VFmaLegacy,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VFmaLegacy,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::F32>;
    type Source1 = crate::ModifiedSource<crate::F32>;
    type Source2 = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Binary<family::VFmac, crate::F16, crate::F16, crate::F16, crate::E32> for () {}
impl crate::SupportedBinary<family::VFmac, crate::F16, crate::F16, crate::F16, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::F16>;
    type Source1 = crate::VectorRegister<crate::F16>;
}
impl crate::private::Binary<family::VFmac, crate::F16, crate::F16, crate::F16, crate::E64> for () {}
impl crate::SupportedBinary<family::VFmac, crate::F16, crate::F16, crate::F16, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::F16>;
    type Source1 = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Binary<family::VFmac, crate::F32, crate::F32, crate::F32, crate::E32> for () {}
impl crate::SupportedBinary<family::VFmac, crate::F32, crate::F32, crate::F32, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::F32>;
    type Source1 = crate::VectorRegister<crate::F32>;
}
impl crate::private::Binary<family::VFmac, crate::F32, crate::F32, crate::F32, crate::E64> for () {}
impl crate::SupportedBinary<family::VFmac, crate::F32, crate::F32, crate::F32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::F32>;
    type Source1 = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Binary<family::VFmacLegacy, crate::F32, crate::F32, crate::F32, crate::E32>
    for ()
{
}
impl crate::SupportedBinary<family::VFmacLegacy, crate::F32, crate::F32, crate::F32, crate::E32>
    for ()
{
    type Source0 = crate::SourceOperand<crate::F32>;
    type Source1 = crate::VectorRegister<crate::F32>;
}
impl crate::private::Binary<family::VFmacLegacy, crate::F32, crate::F32, crate::F32, crate::E64>
    for ()
{
}
impl crate::SupportedBinary<family::VFmacLegacy, crate::F32, crate::F32, crate::F32, crate::E64>
    for ()
{
    type Source0 = crate::ModifiedSource<crate::F32>;
    type Source1 = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Unary<family::VFract, crate::F16, crate::F16, crate::E32> for () {}
impl crate::SupportedUnary<family::VFract, crate::F16, crate::F16, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F16>;
}
impl crate::private::Unary<family::VFract, crate::F16, crate::F16, crate::E64> for () {}
impl crate::SupportedUnary<family::VFract, crate::F16, crate::F16, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Unary<family::VFract, crate::F32, crate::F32, crate::E32> for () {}
impl crate::SupportedUnary<family::VFract, crate::F32, crate::F32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F32>;
}
impl crate::private::Unary<family::VFract, crate::F32, crate::F32, crate::E64> for () {}
impl crate::SupportedUnary<family::VFract, crate::F32, crate::F32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Unary<family::VFract, crate::F64, crate::F64, crate::E32> for () {}
impl crate::SupportedUnary<family::VFract, crate::F64, crate::F64, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F64>;
}
impl crate::private::Unary<family::VFract, crate::F64, crate::F64, crate::E64> for () {}
impl crate::SupportedUnary<family::VFract, crate::F64, crate::F64, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F64>;
}
impl crate::private::Unary<family::VFrexpExp, crate::I16, crate::F16, crate::E32> for () {}
impl crate::SupportedUnary<family::VFrexpExp, crate::I16, crate::F16, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F16>;
}
impl crate::private::Unary<family::VFrexpExp, crate::I16, crate::F16, crate::E64> for () {}
impl crate::SupportedUnary<family::VFrexpExp, crate::I16, crate::F16, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Unary<family::VFrexpExp, crate::I32, crate::F32, crate::E32> for () {}
impl crate::SupportedUnary<family::VFrexpExp, crate::I32, crate::F32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F32>;
}
impl crate::private::Unary<family::VFrexpExp, crate::I32, crate::F32, crate::E64> for () {}
impl crate::SupportedUnary<family::VFrexpExp, crate::I32, crate::F32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Unary<family::VFrexpExp, crate::I32, crate::F64, crate::E32> for () {}
impl crate::SupportedUnary<family::VFrexpExp, crate::I32, crate::F64, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F64>;
}
impl crate::private::Unary<family::VFrexpExp, crate::I32, crate::F64, crate::E64> for () {}
impl crate::SupportedUnary<family::VFrexpExp, crate::I32, crate::F64, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F64>;
}
impl crate::private::Unary<family::VFrexpMant, crate::F16, crate::F16, crate::E32> for () {}
impl crate::SupportedUnary<family::VFrexpMant, crate::F16, crate::F16, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F16>;
}
impl crate::private::Unary<family::VFrexpMant, crate::F16, crate::F16, crate::E64> for () {}
impl crate::SupportedUnary<family::VFrexpMant, crate::F16, crate::F16, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Unary<family::VFrexpMant, crate::F32, crate::F32, crate::E32> for () {}
impl crate::SupportedUnary<family::VFrexpMant, crate::F32, crate::F32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F32>;
}
impl crate::private::Unary<family::VFrexpMant, crate::F32, crate::F32, crate::E64> for () {}
impl crate::SupportedUnary<family::VFrexpMant, crate::F32, crate::F32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Unary<family::VFrexpMant, crate::F64, crate::F64, crate::E32> for () {}
impl crate::SupportedUnary<family::VFrexpMant, crate::F64, crate::F64, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F64>;
}
impl crate::private::Unary<family::VFrexpMant, crate::F64, crate::F64, crate::E64> for () {}
impl crate::SupportedUnary<family::VFrexpMant, crate::F64, crate::F64, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F64>;
}
impl crate::private::Binary<family::VLdexp, crate::F16, crate::F16, crate::F16, crate::E32> for () {}
impl crate::SupportedBinary<family::VLdexp, crate::F16, crate::F16, crate::F16, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::F16>;
    type Source1 = crate::VectorRegister<crate::F16>;
}
impl crate::private::Binary<family::VLdexp, crate::F16, crate::F16, crate::F16, crate::E64> for () {}
impl crate::SupportedBinary<family::VLdexp, crate::F16, crate::F16, crate::F16, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::F16>;
    type Source1 = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Binary<family::VLdexp, crate::F32, crate::F32, crate::I32, crate::E64> for () {}
impl crate::SupportedBinary<family::VLdexp, crate::F32, crate::F32, crate::I32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::F32>;
    type Source1 = crate::ModifiedSource<crate::I32>;
}
impl crate::private::Binary<family::VLdexp, crate::F64, crate::F64, crate::I32, crate::E64> for () {}
impl crate::SupportedBinary<family::VLdexp, crate::F64, crate::F64, crate::I32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::F64>;
    type Source1 = crate::ModifiedSource<crate::I32>;
}
impl
    crate::private::Ternary<
        family::VLerp,
        crate::U32,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VLerp,
        crate::U32,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::B32>;
    type Source1 = crate::ModifiedSource<crate::B32>;
    type Source2 = crate::ModifiedSource<crate::B32>;
}
impl crate::private::Unary<family::VLog, crate::F16, crate::F16, crate::E32> for () {}
impl crate::SupportedUnary<family::VLog, crate::F16, crate::F16, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F16>;
}
impl crate::private::Unary<family::VLog, crate::F16, crate::F16, crate::E64> for () {}
impl crate::SupportedUnary<family::VLog, crate::F16, crate::F16, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Unary<family::VLog, crate::F32, crate::F32, crate::E32> for () {}
impl crate::SupportedUnary<family::VLog, crate::F32, crate::F32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F32>;
}
impl crate::private::Unary<family::VLog, crate::F32, crate::F32, crate::E64> for () {}
impl crate::SupportedUnary<family::VLog, crate::F32, crate::F32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F32>;
}
impl
    crate::private::Ternary<
        family::VLshlAdd,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VLshlAdd,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::U32>;
    type Source1 = crate::ModifiedSource<crate::U32>;
    type Source2 = crate::ModifiedSource<crate::U32>;
}
impl
    crate::private::Ternary<
        family::VLshlOr,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VLshlOr,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::B32>;
    type Source1 = crate::ModifiedSource<crate::B32>;
    type Source2 = crate::ModifiedSource<crate::B32>;
}
impl crate::private::Binary<family::VLshlrev, crate::B16, crate::B16, crate::B16, crate::E64>
    for ()
{
}
impl crate::SupportedBinary<family::VLshlrev, crate::B16, crate::B16, crate::B16, crate::E64>
    for ()
{
    type Source0 = crate::ModifiedSource<crate::B16>;
    type Source1 = crate::ModifiedSource<crate::B16>;
}
impl crate::private::Binary<family::VLshlrev, crate::B32, crate::B32, crate::B32, crate::E32>
    for ()
{
}
impl crate::SupportedBinary<family::VLshlrev, crate::B32, crate::B32, crate::B32, crate::E32>
    for ()
{
    type Source0 = crate::SourceOperand<crate::B32>;
    type Source1 = crate::VectorRegister<crate::B32>;
}
impl crate::private::Binary<family::VLshlrev, crate::B32, crate::B32, crate::B32, crate::E64>
    for ()
{
}
impl crate::SupportedBinary<family::VLshlrev, crate::B32, crate::B32, crate::B32, crate::E64>
    for ()
{
    type Source0 = crate::ModifiedSource<crate::B32>;
    type Source1 = crate::ModifiedSource<crate::B32>;
}
impl crate::private::Binary<family::VLshlrev, crate::B64, crate::U32, crate::B64, crate::E64>
    for ()
{
}
impl crate::SupportedBinary<family::VLshlrev, crate::B64, crate::U32, crate::B64, crate::E64>
    for ()
{
    type Source0 = crate::ModifiedSource<crate::U32>;
    type Source1 = crate::ModifiedSource<crate::B64>;
}
impl crate::private::Binary<family::VLshrrev, crate::B16, crate::B16, crate::B16, crate::E64>
    for ()
{
}
impl crate::SupportedBinary<family::VLshrrev, crate::B16, crate::B16, crate::B16, crate::E64>
    for ()
{
    type Source0 = crate::ModifiedSource<crate::B16>;
    type Source1 = crate::ModifiedSource<crate::B16>;
}
impl crate::private::Binary<family::VLshrrev, crate::B32, crate::B32, crate::B32, crate::E32>
    for ()
{
}
impl crate::SupportedBinary<family::VLshrrev, crate::B32, crate::B32, crate::B32, crate::E32>
    for ()
{
    type Source0 = crate::SourceOperand<crate::B32>;
    type Source1 = crate::VectorRegister<crate::B32>;
}
impl crate::private::Binary<family::VLshrrev, crate::B32, crate::B32, crate::B32, crate::E64>
    for ()
{
}
impl crate::SupportedBinary<family::VLshrrev, crate::B32, crate::B32, crate::B32, crate::E64>
    for ()
{
    type Source0 = crate::ModifiedSource<crate::B32>;
    type Source1 = crate::ModifiedSource<crate::B32>;
}
impl crate::private::Binary<family::VLshrrev, crate::B64, crate::U32, crate::B64, crate::E64>
    for ()
{
}
impl crate::SupportedBinary<family::VLshrrev, crate::B64, crate::U32, crate::B64, crate::E64>
    for ()
{
    type Source0 = crate::ModifiedSource<crate::U32>;
    type Source1 = crate::ModifiedSource<crate::B64>;
}
impl
    crate::private::Ternary<
        family::VMad,
        crate::I16,
        crate::I16,
        crate::I16,
        crate::I16,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMad,
        crate::I16,
        crate::I16,
        crate::I16,
        crate::I16,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::I16>;
    type Source1 = crate::ModifiedSource<crate::I16>;
    type Source2 = crate::ModifiedSource<crate::I16>;
}
impl
    crate::private::Ternary<
        family::VMad,
        crate::I32,
        crate::I16,
        crate::I16,
        crate::I32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMad,
        crate::I32,
        crate::I16,
        crate::I16,
        crate::I32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::I16>;
    type Source1 = crate::ModifiedSource<crate::I16>;
    type Source2 = crate::ModifiedSource<crate::I32>;
}
impl
    crate::private::Ternary<
        family::VMad,
        crate::U16,
        crate::U16,
        crate::U16,
        crate::U16,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMad,
        crate::U16,
        crate::U16,
        crate::U16,
        crate::U16,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::U16>;
    type Source1 = crate::ModifiedSource<crate::U16>;
    type Source2 = crate::ModifiedSource<crate::U16>;
}
impl
    crate::private::Ternary<
        family::VMad,
        crate::U32,
        crate::U16,
        crate::U16,
        crate::U32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMad,
        crate::U32,
        crate::U16,
        crate::U16,
        crate::U32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::U16>;
    type Source1 = crate::ModifiedSource<crate::U16>;
    type Source2 = crate::ModifiedSource<crate::U32>;
}
impl
    crate::private::Ternary<
        family::VMax3,
        crate::F16,
        crate::F16,
        crate::F16,
        crate::F16,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMax3,
        crate::F16,
        crate::F16,
        crate::F16,
        crate::F16,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::F16>;
    type Source1 = crate::ModifiedSource<crate::F16>;
    type Source2 = crate::ModifiedSource<crate::F16>;
}
impl
    crate::private::Ternary<
        family::VMax3,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMax3,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::F32>;
    type Source1 = crate::ModifiedSource<crate::F32>;
    type Source2 = crate::ModifiedSource<crate::F32>;
}
impl
    crate::private::Ternary<
        family::VMax3,
        crate::I16,
        crate::I16,
        crate::I16,
        crate::I16,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMax3,
        crate::I16,
        crate::I16,
        crate::I16,
        crate::I16,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::I16>;
    type Source1 = crate::ModifiedSource<crate::I16>;
    type Source2 = crate::ModifiedSource<crate::I16>;
}
impl
    crate::private::Ternary<
        family::VMax3,
        crate::I32,
        crate::I32,
        crate::I32,
        crate::I32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMax3,
        crate::I32,
        crate::I32,
        crate::I32,
        crate::I32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::I32>;
    type Source1 = crate::ModifiedSource<crate::I32>;
    type Source2 = crate::ModifiedSource<crate::I32>;
}
impl
    crate::private::Ternary<
        family::VMax3,
        crate::U16,
        crate::U16,
        crate::U16,
        crate::U16,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMax3,
        crate::U16,
        crate::U16,
        crate::U16,
        crate::U16,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::U16>;
    type Source1 = crate::ModifiedSource<crate::U16>;
    type Source2 = crate::ModifiedSource<crate::U16>;
}
impl
    crate::private::Ternary<
        family::VMax3,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMax3,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::U32>;
    type Source1 = crate::ModifiedSource<crate::U32>;
    type Source2 = crate::ModifiedSource<crate::U32>;
}
impl crate::private::Binary<family::VMax, crate::F16, crate::F16, crate::F16, crate::E32> for () {}
impl crate::SupportedBinary<family::VMax, crate::F16, crate::F16, crate::F16, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::F16>;
    type Source1 = crate::VectorRegister<crate::F16>;
}
impl crate::private::Binary<family::VMax, crate::F16, crate::F16, crate::F16, crate::E64> for () {}
impl crate::SupportedBinary<family::VMax, crate::F16, crate::F16, crate::F16, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::F16>;
    type Source1 = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Binary<family::VMax, crate::F32, crate::F32, crate::F32, crate::E32> for () {}
impl crate::SupportedBinary<family::VMax, crate::F32, crate::F32, crate::F32, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::F32>;
    type Source1 = crate::VectorRegister<crate::F32>;
}
impl crate::private::Binary<family::VMax, crate::F32, crate::F32, crate::F32, crate::E64> for () {}
impl crate::SupportedBinary<family::VMax, crate::F32, crate::F32, crate::F32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::F32>;
    type Source1 = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Binary<family::VMax, crate::F64, crate::F64, crate::F64, crate::E64> for () {}
impl crate::SupportedBinary<family::VMax, crate::F64, crate::F64, crate::F64, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::F64>;
    type Source1 = crate::ModifiedSource<crate::F64>;
}
impl crate::private::Binary<family::VMax, crate::I16, crate::I16, crate::I16, crate::E64> for () {}
impl crate::SupportedBinary<family::VMax, crate::I16, crate::I16, crate::I16, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::I16>;
    type Source1 = crate::ModifiedSource<crate::I16>;
}
impl crate::private::Binary<family::VMax, crate::I32, crate::I32, crate::I32, crate::E32> for () {}
impl crate::SupportedBinary<family::VMax, crate::I32, crate::I32, crate::I32, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::I32>;
    type Source1 = crate::VectorRegister<crate::I32>;
}
impl crate::private::Binary<family::VMax, crate::I32, crate::I32, crate::I32, crate::E64> for () {}
impl crate::SupportedBinary<family::VMax, crate::I32, crate::I32, crate::I32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::I32>;
    type Source1 = crate::ModifiedSource<crate::I32>;
}
impl crate::private::Binary<family::VMax, crate::U16, crate::U16, crate::U16, crate::E64> for () {}
impl crate::SupportedBinary<family::VMax, crate::U16, crate::U16, crate::U16, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::U16>;
    type Source1 = crate::ModifiedSource<crate::U16>;
}
impl crate::private::Binary<family::VMax, crate::U32, crate::U32, crate::U32, crate::E32> for () {}
impl crate::SupportedBinary<family::VMax, crate::U32, crate::U32, crate::U32, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::U32>;
    type Source1 = crate::VectorRegister<crate::U32>;
}
impl crate::private::Binary<family::VMax, crate::U32, crate::U32, crate::U32, crate::E64> for () {}
impl crate::SupportedBinary<family::VMax, crate::U32, crate::U32, crate::U32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::U32>;
    type Source1 = crate::ModifiedSource<crate::U32>;
}
impl crate::private::Binary<family::VMbcntHi, crate::B32, crate::B32, crate::B32, crate::E64>
    for ()
{
}
impl crate::SupportedBinary<family::VMbcntHi, crate::B32, crate::B32, crate::B32, crate::E64>
    for ()
{
    type Source0 = crate::ModifiedSource<crate::B32>;
    type Source1 = crate::ModifiedSource<crate::B32>;
}
impl crate::private::Binary<family::VMbcntLo, crate::B32, crate::B32, crate::B32, crate::E64>
    for ()
{
}
impl crate::SupportedBinary<family::VMbcntLo, crate::B32, crate::B32, crate::B32, crate::E64>
    for ()
{
    type Source0 = crate::ModifiedSource<crate::B32>;
    type Source1 = crate::ModifiedSource<crate::B32>;
}
impl
    crate::private::Ternary<
        family::VMed3,
        crate::F16,
        crate::F16,
        crate::F16,
        crate::F16,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMed3,
        crate::F16,
        crate::F16,
        crate::F16,
        crate::F16,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::F16>;
    type Source1 = crate::ModifiedSource<crate::F16>;
    type Source2 = crate::ModifiedSource<crate::F16>;
}
impl
    crate::private::Ternary<
        family::VMed3,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMed3,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::F32>;
    type Source1 = crate::ModifiedSource<crate::F32>;
    type Source2 = crate::ModifiedSource<crate::F32>;
}
impl
    crate::private::Ternary<
        family::VMed3,
        crate::I16,
        crate::I16,
        crate::I16,
        crate::I16,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMed3,
        crate::I16,
        crate::I16,
        crate::I16,
        crate::I16,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::I16>;
    type Source1 = crate::ModifiedSource<crate::I16>;
    type Source2 = crate::ModifiedSource<crate::I16>;
}
impl
    crate::private::Ternary<
        family::VMed3,
        crate::I32,
        crate::I32,
        crate::I32,
        crate::I32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMed3,
        crate::I32,
        crate::I32,
        crate::I32,
        crate::I32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::I32>;
    type Source1 = crate::ModifiedSource<crate::I32>;
    type Source2 = crate::ModifiedSource<crate::I32>;
}
impl
    crate::private::Ternary<
        family::VMed3,
        crate::U16,
        crate::U16,
        crate::U16,
        crate::U16,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMed3,
        crate::U16,
        crate::U16,
        crate::U16,
        crate::U16,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::U16>;
    type Source1 = crate::ModifiedSource<crate::U16>;
    type Source2 = crate::ModifiedSource<crate::U16>;
}
impl
    crate::private::Ternary<
        family::VMed3,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMed3,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::U32>;
    type Source1 = crate::ModifiedSource<crate::U32>;
    type Source2 = crate::ModifiedSource<crate::U32>;
}
impl
    crate::private::Ternary<
        family::VMin3,
        crate::F16,
        crate::F16,
        crate::F16,
        crate::F16,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMin3,
        crate::F16,
        crate::F16,
        crate::F16,
        crate::F16,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::F16>;
    type Source1 = crate::ModifiedSource<crate::F16>;
    type Source2 = crate::ModifiedSource<crate::F16>;
}
impl
    crate::private::Ternary<
        family::VMin3,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMin3,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::F32>;
    type Source1 = crate::ModifiedSource<crate::F32>;
    type Source2 = crate::ModifiedSource<crate::F32>;
}
impl
    crate::private::Ternary<
        family::VMin3,
        crate::I16,
        crate::I16,
        crate::I16,
        crate::I16,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMin3,
        crate::I16,
        crate::I16,
        crate::I16,
        crate::I16,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::I16>;
    type Source1 = crate::ModifiedSource<crate::I16>;
    type Source2 = crate::ModifiedSource<crate::I16>;
}
impl
    crate::private::Ternary<
        family::VMin3,
        crate::I32,
        crate::I32,
        crate::I32,
        crate::I32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMin3,
        crate::I32,
        crate::I32,
        crate::I32,
        crate::I32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::I32>;
    type Source1 = crate::ModifiedSource<crate::I32>;
    type Source2 = crate::ModifiedSource<crate::I32>;
}
impl
    crate::private::Ternary<
        family::VMin3,
        crate::U16,
        crate::U16,
        crate::U16,
        crate::U16,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMin3,
        crate::U16,
        crate::U16,
        crate::U16,
        crate::U16,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::U16>;
    type Source1 = crate::ModifiedSource<crate::U16>;
    type Source2 = crate::ModifiedSource<crate::U16>;
}
impl
    crate::private::Ternary<
        family::VMin3,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMin3,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::U32>;
    type Source1 = crate::ModifiedSource<crate::U32>;
    type Source2 = crate::ModifiedSource<crate::U32>;
}
impl crate::private::Binary<family::VMin, crate::F16, crate::F16, crate::F16, crate::E32> for () {}
impl crate::SupportedBinary<family::VMin, crate::F16, crate::F16, crate::F16, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::F16>;
    type Source1 = crate::VectorRegister<crate::F16>;
}
impl crate::private::Binary<family::VMin, crate::F16, crate::F16, crate::F16, crate::E64> for () {}
impl crate::SupportedBinary<family::VMin, crate::F16, crate::F16, crate::F16, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::F16>;
    type Source1 = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Binary<family::VMin, crate::F32, crate::F32, crate::F32, crate::E32> for () {}
impl crate::SupportedBinary<family::VMin, crate::F32, crate::F32, crate::F32, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::F32>;
    type Source1 = crate::VectorRegister<crate::F32>;
}
impl crate::private::Binary<family::VMin, crate::F32, crate::F32, crate::F32, crate::E64> for () {}
impl crate::SupportedBinary<family::VMin, crate::F32, crate::F32, crate::F32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::F32>;
    type Source1 = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Binary<family::VMin, crate::F64, crate::F64, crate::F64, crate::E64> for () {}
impl crate::SupportedBinary<family::VMin, crate::F64, crate::F64, crate::F64, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::F64>;
    type Source1 = crate::ModifiedSource<crate::F64>;
}
impl crate::private::Binary<family::VMin, crate::I16, crate::I16, crate::I16, crate::E64> for () {}
impl crate::SupportedBinary<family::VMin, crate::I16, crate::I16, crate::I16, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::I16>;
    type Source1 = crate::ModifiedSource<crate::I16>;
}
impl crate::private::Binary<family::VMin, crate::I32, crate::I32, crate::I32, crate::E32> for () {}
impl crate::SupportedBinary<family::VMin, crate::I32, crate::I32, crate::I32, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::I32>;
    type Source1 = crate::VectorRegister<crate::I32>;
}
impl crate::private::Binary<family::VMin, crate::I32, crate::I32, crate::I32, crate::E64> for () {}
impl crate::SupportedBinary<family::VMin, crate::I32, crate::I32, crate::I32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::I32>;
    type Source1 = crate::ModifiedSource<crate::I32>;
}
impl crate::private::Binary<family::VMin, crate::U16, crate::U16, crate::U16, crate::E64> for () {}
impl crate::SupportedBinary<family::VMin, crate::U16, crate::U16, crate::U16, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::U16>;
    type Source1 = crate::ModifiedSource<crate::U16>;
}
impl crate::private::Binary<family::VMin, crate::U32, crate::U32, crate::U32, crate::E32> for () {}
impl crate::SupportedBinary<family::VMin, crate::U32, crate::U32, crate::U32, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::U32>;
    type Source1 = crate::VectorRegister<crate::U32>;
}
impl crate::private::Binary<family::VMin, crate::U32, crate::U32, crate::U32, crate::E64> for () {}
impl crate::SupportedBinary<family::VMin, crate::U32, crate::U32, crate::U32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::U32>;
    type Source1 = crate::ModifiedSource<crate::U32>;
}
impl crate::private::Unary<family::VMov, crate::B32, crate::B32, crate::E32> for () {}
impl crate::SupportedUnary<family::VMov, crate::B32, crate::B32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::B32>;
}
impl crate::private::Unary<family::VMov, crate::B32, crate::B32, crate::E64> for () {}
impl crate::SupportedUnary<family::VMov, crate::B32, crate::B32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::B32>;
}
impl
    crate::private::Ternary<
        family::VMqsadPk,
        crate::B64,
        crate::B64,
        crate::B32,
        crate::B64,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMqsadPk,
        crate::B64,
        crate::B64,
        crate::B32,
        crate::B64,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::B64>;
    type Source1 = crate::ModifiedSource<crate::B32>;
    type Source2 = crate::ModifiedSource<crate::B64>;
}
impl
    crate::private::Ternary<
        family::VMsad,
        crate::U32,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMsad,
        crate::U32,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::B32>;
    type Source1 = crate::ModifiedSource<crate::B32>;
    type Source2 = crate::ModifiedSource<crate::B32>;
}
impl crate::private::Binary<family::VMul, crate::F16, crate::F16, crate::F16, crate::E32> for () {}
impl crate::SupportedBinary<family::VMul, crate::F16, crate::F16, crate::F16, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::F16>;
    type Source1 = crate::VectorRegister<crate::F16>;
}
impl crate::private::Binary<family::VMul, crate::F16, crate::F16, crate::F16, crate::E64> for () {}
impl crate::SupportedBinary<family::VMul, crate::F16, crate::F16, crate::F16, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::F16>;
    type Source1 = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Binary<family::VMul, crate::F32, crate::F32, crate::F32, crate::E32> for () {}
impl crate::SupportedBinary<family::VMul, crate::F32, crate::F32, crate::F32, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::F32>;
    type Source1 = crate::VectorRegister<crate::F32>;
}
impl crate::private::Binary<family::VMul, crate::F32, crate::F32, crate::F32, crate::E64> for () {}
impl crate::SupportedBinary<family::VMul, crate::F32, crate::F32, crate::F32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::F32>;
    type Source1 = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Binary<family::VMul, crate::F64, crate::F64, crate::F64, crate::E64> for () {}
impl crate::SupportedBinary<family::VMul, crate::F64, crate::F64, crate::F64, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::F64>;
    type Source1 = crate::ModifiedSource<crate::F64>;
}
impl crate::private::Binary<family::VMulHi, crate::I32, crate::I32, crate::I32, crate::E64> for () {}
impl crate::SupportedBinary<family::VMulHi, crate::I32, crate::I32, crate::I32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::I32>;
    type Source1 = crate::ModifiedSource<crate::I32>;
}
impl crate::private::Binary<family::VMulHi, crate::U32, crate::U32, crate::U32, crate::E64> for () {}
impl crate::SupportedBinary<family::VMulHi, crate::U32, crate::U32, crate::U32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::U32>;
    type Source1 = crate::ModifiedSource<crate::U32>;
}
impl crate::private::Binary<family::VMulLegacy, crate::F32, crate::F32, crate::F32, crate::E32>
    for ()
{
}
impl crate::SupportedBinary<family::VMulLegacy, crate::F32, crate::F32, crate::F32, crate::E32>
    for ()
{
    type Source0 = crate::SourceOperand<crate::F32>;
    type Source1 = crate::VectorRegister<crate::F32>;
}
impl crate::private::Binary<family::VMulLegacy, crate::F32, crate::F32, crate::F32, crate::E64>
    for ()
{
}
impl crate::SupportedBinary<family::VMulLegacy, crate::F32, crate::F32, crate::F32, crate::E64>
    for ()
{
    type Source0 = crate::ModifiedSource<crate::F32>;
    type Source1 = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Binary<family::VMulLo, crate::U16, crate::U16, crate::U16, crate::E64> for () {}
impl crate::SupportedBinary<family::VMulLo, crate::U16, crate::U16, crate::U16, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::U16>;
    type Source1 = crate::ModifiedSource<crate::U16>;
}
impl crate::private::Binary<family::VMulLo, crate::U32, crate::U32, crate::U32, crate::E64> for () {}
impl crate::SupportedBinary<family::VMulLo, crate::U32, crate::U32, crate::U32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::U32>;
    type Source1 = crate::ModifiedSource<crate::U32>;
}
impl
    crate::private::Ternary<
        family::VMullit,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VMullit,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::F32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::F32>;
    type Source1 = crate::ModifiedSource<crate::F32>;
    type Source2 = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Unary<family::VNot, crate::B32, crate::B32, crate::E32> for () {}
impl crate::SupportedUnary<family::VNot, crate::B32, crate::B32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::B32>;
}
impl crate::private::Unary<family::VNot, crate::B32, crate::B32, crate::E64> for () {}
impl crate::SupportedUnary<family::VNot, crate::B32, crate::B32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::B32>;
}
impl
    crate::private::Ternary<
        family::VOr3,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VOr3,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::B32>;
    type Source1 = crate::ModifiedSource<crate::B32>;
    type Source2 = crate::ModifiedSource<crate::B32>;
}
impl crate::private::Binary<family::VOr, crate::B32, crate::B32, crate::B32, crate::E32> for () {}
impl crate::SupportedBinary<family::VOr, crate::B32, crate::B32, crate::B32, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::B32>;
    type Source1 = crate::VectorRegister<crate::B32>;
}
impl crate::private::Binary<family::VOr, crate::B32, crate::B32, crate::B32, crate::E64> for () {}
impl crate::SupportedBinary<family::VOr, crate::B32, crate::B32, crate::B32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::B32>;
    type Source1 = crate::ModifiedSource<crate::B32>;
}
impl crate::private::Binary<family::VPack, crate::B32, crate::F16, crate::F16, crate::E64> for () {}
impl crate::SupportedBinary<family::VPack, crate::B32, crate::F16, crate::F16, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::F16>;
    type Source1 = crate::ModifiedSource<crate::F16>;
}
impl
    crate::private::Ternary<
        family::VQsadPk,
        crate::B64,
        crate::B64,
        crate::B32,
        crate::B64,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VQsadPk,
        crate::B64,
        crate::B64,
        crate::B32,
        crate::B64,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::B64>;
    type Source1 = crate::ModifiedSource<crate::B32>;
    type Source2 = crate::ModifiedSource<crate::B64>;
}
impl crate::private::Unary<family::VRcp, crate::F16, crate::F16, crate::E32> for () {}
impl crate::SupportedUnary<family::VRcp, crate::F16, crate::F16, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F16>;
}
impl crate::private::Unary<family::VRcp, crate::F16, crate::F16, crate::E64> for () {}
impl crate::SupportedUnary<family::VRcp, crate::F16, crate::F16, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Unary<family::VRcp, crate::F32, crate::F32, crate::E32> for () {}
impl crate::SupportedUnary<family::VRcp, crate::F32, crate::F32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F32>;
}
impl crate::private::Unary<family::VRcp, crate::F32, crate::F32, crate::E64> for () {}
impl crate::SupportedUnary<family::VRcp, crate::F32, crate::F32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Unary<family::VRcp, crate::F64, crate::F64, crate::E32> for () {}
impl crate::SupportedUnary<family::VRcp, crate::F64, crate::F64, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F64>;
}
impl crate::private::Unary<family::VRcp, crate::F64, crate::F64, crate::E64> for () {}
impl crate::SupportedUnary<family::VRcp, crate::F64, crate::F64, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F64>;
}
impl crate::private::Unary<family::VRcpIflag, crate::F32, crate::F32, crate::E32> for () {}
impl crate::SupportedUnary<family::VRcpIflag, crate::F32, crate::F32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F32>;
}
impl crate::private::Unary<family::VRcpIflag, crate::F32, crate::F32, crate::E64> for () {}
impl crate::SupportedUnary<family::VRcpIflag, crate::F32, crate::F32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Unary<family::VRndne, crate::F16, crate::F16, crate::E32> for () {}
impl crate::SupportedUnary<family::VRndne, crate::F16, crate::F16, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F16>;
}
impl crate::private::Unary<family::VRndne, crate::F16, crate::F16, crate::E64> for () {}
impl crate::SupportedUnary<family::VRndne, crate::F16, crate::F16, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Unary<family::VRndne, crate::F32, crate::F32, crate::E32> for () {}
impl crate::SupportedUnary<family::VRndne, crate::F32, crate::F32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F32>;
}
impl crate::private::Unary<family::VRndne, crate::F32, crate::F32, crate::E64> for () {}
impl crate::SupportedUnary<family::VRndne, crate::F32, crate::F32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Unary<family::VRndne, crate::F64, crate::F64, crate::E32> for () {}
impl crate::SupportedUnary<family::VRndne, crate::F64, crate::F64, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F64>;
}
impl crate::private::Unary<family::VRndne, crate::F64, crate::F64, crate::E64> for () {}
impl crate::SupportedUnary<family::VRndne, crate::F64, crate::F64, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F64>;
}
impl crate::private::Unary<family::VRsq, crate::F16, crate::F16, crate::E32> for () {}
impl crate::SupportedUnary<family::VRsq, crate::F16, crate::F16, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F16>;
}
impl crate::private::Unary<family::VRsq, crate::F16, crate::F16, crate::E64> for () {}
impl crate::SupportedUnary<family::VRsq, crate::F16, crate::F16, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Unary<family::VRsq, crate::F32, crate::F32, crate::E32> for () {}
impl crate::SupportedUnary<family::VRsq, crate::F32, crate::F32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F32>;
}
impl crate::private::Unary<family::VRsq, crate::F32, crate::F32, crate::E64> for () {}
impl crate::SupportedUnary<family::VRsq, crate::F32, crate::F32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Unary<family::VRsq, crate::F64, crate::F64, crate::E32> for () {}
impl crate::SupportedUnary<family::VRsq, crate::F64, crate::F64, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F64>;
}
impl crate::private::Unary<family::VRsq, crate::F64, crate::F64, crate::E64> for () {}
impl crate::SupportedUnary<family::VRsq, crate::F64, crate::F64, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F64>;
}
impl
    crate::private::Ternary<
        family::VSad,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VSad,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::U32>;
    type Source1 = crate::ModifiedSource<crate::U32>;
    type Source2 = crate::ModifiedSource<crate::U32>;
}
impl crate::private::Unary<family::VSin, crate::F16, crate::F16, crate::E32> for () {}
impl crate::SupportedUnary<family::VSin, crate::F16, crate::F16, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F16>;
}
impl crate::private::Unary<family::VSin, crate::F16, crate::F16, crate::E64> for () {}
impl crate::SupportedUnary<family::VSin, crate::F16, crate::F16, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Unary<family::VSin, crate::F32, crate::F32, crate::E32> for () {}
impl crate::SupportedUnary<family::VSin, crate::F32, crate::F32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F32>;
}
impl crate::private::Unary<family::VSin, crate::F32, crate::F32, crate::E64> for () {}
impl crate::SupportedUnary<family::VSin, crate::F32, crate::F32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Unary<family::VSqrt, crate::F16, crate::F16, crate::E32> for () {}
impl crate::SupportedUnary<family::VSqrt, crate::F16, crate::F16, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F16>;
}
impl crate::private::Unary<family::VSqrt, crate::F16, crate::F16, crate::E64> for () {}
impl crate::SupportedUnary<family::VSqrt, crate::F16, crate::F16, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Unary<family::VSqrt, crate::F32, crate::F32, crate::E32> for () {}
impl crate::SupportedUnary<family::VSqrt, crate::F32, crate::F32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F32>;
}
impl crate::private::Unary<family::VSqrt, crate::F32, crate::F32, crate::E64> for () {}
impl crate::SupportedUnary<family::VSqrt, crate::F32, crate::F32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Unary<family::VSqrt, crate::F64, crate::F64, crate::E32> for () {}
impl crate::SupportedUnary<family::VSqrt, crate::F64, crate::F64, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F64>;
}
impl crate::private::Unary<family::VSqrt, crate::F64, crate::F64, crate::E64> for () {}
impl crate::SupportedUnary<family::VSqrt, crate::F64, crate::F64, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F64>;
}
impl crate::private::Binary<family::VSub, crate::F16, crate::F16, crate::F16, crate::E32> for () {}
impl crate::SupportedBinary<family::VSub, crate::F16, crate::F16, crate::F16, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::F16>;
    type Source1 = crate::VectorRegister<crate::F16>;
}
impl crate::private::Binary<family::VSub, crate::F16, crate::F16, crate::F16, crate::E64> for () {}
impl crate::SupportedBinary<family::VSub, crate::F16, crate::F16, crate::F16, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::F16>;
    type Source1 = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Binary<family::VSub, crate::F32, crate::F32, crate::F32, crate::E32> for () {}
impl crate::SupportedBinary<family::VSub, crate::F32, crate::F32, crate::F32, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::F32>;
    type Source1 = crate::VectorRegister<crate::F32>;
}
impl crate::private::Binary<family::VSub, crate::F32, crate::F32, crate::F32, crate::E64> for () {}
impl crate::SupportedBinary<family::VSub, crate::F32, crate::F32, crate::F32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::F32>;
    type Source1 = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Binary<family::VSubNc, crate::I16, crate::I16, crate::I16, crate::E64> for () {}
impl crate::SupportedBinary<family::VSubNc, crate::I16, crate::I16, crate::I16, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::I16>;
    type Source1 = crate::ModifiedSource<crate::I16>;
}
impl crate::private::Binary<family::VSubNc, crate::I32, crate::I32, crate::I32, crate::E64> for () {}
impl crate::SupportedBinary<family::VSubNc, crate::I32, crate::I32, crate::I32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::I32>;
    type Source1 = crate::ModifiedSource<crate::I32>;
}
impl crate::private::Binary<family::VSubNc, crate::U16, crate::U16, crate::U16, crate::E64> for () {}
impl crate::SupportedBinary<family::VSubNc, crate::U16, crate::U16, crate::U16, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::U16>;
    type Source1 = crate::ModifiedSource<crate::U16>;
}
impl crate::private::Binary<family::VSubNc, crate::U32, crate::U32, crate::U32, crate::E32> for () {}
impl crate::SupportedBinary<family::VSubNc, crate::U32, crate::U32, crate::U32, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::U32>;
    type Source1 = crate::VectorRegister<crate::U32>;
}
impl crate::private::Binary<family::VSubNc, crate::U32, crate::U32, crate::U32, crate::E64> for () {}
impl crate::SupportedBinary<family::VSubNc, crate::U32, crate::U32, crate::U32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::U32>;
    type Source1 = crate::ModifiedSource<crate::U32>;
}
impl crate::private::Binary<family::VSubrev, crate::F16, crate::F16, crate::F16, crate::E32>
    for ()
{
}
impl crate::SupportedBinary<family::VSubrev, crate::F16, crate::F16, crate::F16, crate::E32>
    for ()
{
    type Source0 = crate::SourceOperand<crate::F16>;
    type Source1 = crate::VectorRegister<crate::F16>;
}
impl crate::private::Binary<family::VSubrev, crate::F16, crate::F16, crate::F16, crate::E64>
    for ()
{
}
impl crate::SupportedBinary<family::VSubrev, crate::F16, crate::F16, crate::F16, crate::E64>
    for ()
{
    type Source0 = crate::ModifiedSource<crate::F16>;
    type Source1 = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Binary<family::VSubrev, crate::F32, crate::F32, crate::F32, crate::E32>
    for ()
{
}
impl crate::SupportedBinary<family::VSubrev, crate::F32, crate::F32, crate::F32, crate::E32>
    for ()
{
    type Source0 = crate::SourceOperand<crate::F32>;
    type Source1 = crate::VectorRegister<crate::F32>;
}
impl crate::private::Binary<family::VSubrev, crate::F32, crate::F32, crate::F32, crate::E64>
    for ()
{
}
impl crate::SupportedBinary<family::VSubrev, crate::F32, crate::F32, crate::F32, crate::E64>
    for ()
{
    type Source0 = crate::ModifiedSource<crate::F32>;
    type Source1 = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Binary<family::VSubrevNc, crate::U32, crate::U32, crate::U32, crate::E32>
    for ()
{
}
impl crate::SupportedBinary<family::VSubrevNc, crate::U32, crate::U32, crate::U32, crate::E32>
    for ()
{
    type Source0 = crate::SourceOperand<crate::U32>;
    type Source1 = crate::VectorRegister<crate::U32>;
}
impl crate::private::Binary<family::VSubrevNc, crate::U32, crate::U32, crate::U32, crate::E64>
    for ()
{
}
impl crate::SupportedBinary<family::VSubrevNc, crate::U32, crate::U32, crate::U32, crate::E64>
    for ()
{
    type Source0 = crate::ModifiedSource<crate::U32>;
    type Source1 = crate::ModifiedSource<crate::U32>;
}
impl crate::private::Binary<family::VTrigPreop, crate::F64, crate::F64, crate::B32, crate::E64>
    for ()
{
}
impl crate::SupportedBinary<family::VTrigPreop, crate::F64, crate::F64, crate::B32, crate::E64>
    for ()
{
    type Source0 = crate::ModifiedSource<crate::F64>;
    type Source1 = crate::ModifiedSource<crate::B32>;
}
impl crate::private::Unary<family::VTrunc, crate::F16, crate::F16, crate::E32> for () {}
impl crate::SupportedUnary<family::VTrunc, crate::F16, crate::F16, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F16>;
}
impl crate::private::Unary<family::VTrunc, crate::F16, crate::F16, crate::E64> for () {}
impl crate::SupportedUnary<family::VTrunc, crate::F16, crate::F16, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F16>;
}
impl crate::private::Unary<family::VTrunc, crate::F32, crate::F32, crate::E32> for () {}
impl crate::SupportedUnary<family::VTrunc, crate::F32, crate::F32, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F32>;
}
impl crate::private::Unary<family::VTrunc, crate::F32, crate::F32, crate::E64> for () {}
impl crate::SupportedUnary<family::VTrunc, crate::F32, crate::F32, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F32>;
}
impl crate::private::Unary<family::VTrunc, crate::F64, crate::F64, crate::E32> for () {}
impl crate::SupportedUnary<family::VTrunc, crate::F64, crate::F64, crate::E32> for () {
    type Source = crate::SourceOperand<crate::F64>;
}
impl crate::private::Unary<family::VTrunc, crate::F64, crate::F64, crate::E64> for () {}
impl crate::SupportedUnary<family::VTrunc, crate::F64, crate::F64, crate::E64> for () {
    type Source = crate::ModifiedSource<crate::F64>;
}
impl
    crate::private::Ternary<
        family::VXad,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VXad,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::U32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::U32>;
    type Source1 = crate::ModifiedSource<crate::U32>;
    type Source2 = crate::ModifiedSource<crate::U32>;
}
impl crate::private::Binary<family::VXnor, crate::B32, crate::B32, crate::B32, crate::E32> for () {}
impl crate::SupportedBinary<family::VXnor, crate::B32, crate::B32, crate::B32, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::B32>;
    type Source1 = crate::VectorRegister<crate::B32>;
}
impl crate::private::Binary<family::VXnor, crate::B32, crate::B32, crate::B32, crate::E64> for () {}
impl crate::SupportedBinary<family::VXnor, crate::B32, crate::B32, crate::B32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::B32>;
    type Source1 = crate::ModifiedSource<crate::B32>;
}
impl
    crate::private::Ternary<
        family::VXor3,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::E64,
    > for ()
{
}
impl
    crate::SupportedTernary<
        family::VXor3,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::B32,
        crate::E64,
    > for ()
{
    type Source0 = crate::ModifiedSource<crate::B32>;
    type Source1 = crate::ModifiedSource<crate::B32>;
    type Source2 = crate::ModifiedSource<crate::B32>;
}
impl crate::private::Binary<family::VXor, crate::B32, crate::B32, crate::B32, crate::E32> for () {}
impl crate::SupportedBinary<family::VXor, crate::B32, crate::B32, crate::B32, crate::E32> for () {
    type Source0 = crate::SourceOperand<crate::B32>;
    type Source1 = crate::VectorRegister<crate::B32>;
}
impl crate::private::Binary<family::VXor, crate::B32, crate::B32, crate::B32, crate::E64> for () {}
impl crate::SupportedBinary<family::VXor, crate::B32, crate::B32, crate::B32, crate::E64> for () {
    type Source0 = crate::ModifiedSource<crate::B32>;
    type Source1 = crate::ModifiedSource<crate::B32>;
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DecodedInstruction {
    /// Add three unsigned inputs and store the result into a vector register. No carry-in or carry-out support.
    VAdd3U32E64(
        crate::VectorTernary<
            family::VAdd3,
            crate::U32,
            crate::U32,
            crate::U32,
            crate::U32,
            crate::E64,
        >,
    ),
    /// Add two floating point inputs and store the result into a vector register.
    VAddF16E32(crate::VectorBinary<family::VAdd, crate::F16, crate::F16, crate::F16, crate::E32>),
    /// Add two floating point inputs and store the result into a vector register.
    VAddF16E64(crate::VectorBinary<family::VAdd, crate::F16, crate::F16, crate::F16, crate::E64>),
    /// Add two floating point inputs and store the result into a vector register.
    VAddF32E32(crate::VectorBinary<family::VAdd, crate::F32, crate::F32, crate::F32, crate::E32>),
    /// Add two floating point inputs and store the result into a vector register.
    VAddF32E64(crate::VectorBinary<family::VAdd, crate::F32, crate::F32, crate::F32, crate::E64>),
    /// Add two floating point inputs and store the result into a vector register.
    VAddF64E64(crate::VectorBinary<family::VAdd, crate::F64, crate::F64, crate::F64, crate::E64>),
    /// Add the first two integer inputs, then given a shift count in the third input, calculate the logical shift left of the intermediate result, then store the final result into a vector register.
    VAddLshlU32E64(
        crate::VectorTernary<
            family::VAddLshl,
            crate::U32,
            crate::U32,
            crate::U32,
            crate::U32,
            crate::E64,
        >,
    ),
    /// Add two signed 16-bit integer inputs and store the result into a vector register. No carry-in or carry-out support.
    VAddNcI16E64(
        crate::VectorBinary<family::VAddNc, crate::I16, crate::I16, crate::I16, crate::E64>,
    ),
    /// Add two signed 32-bit integer inputs and store the result into a vector register. No carry-in or carry-out support.
    VAddNcI32E64(
        crate::VectorBinary<family::VAddNc, crate::I32, crate::I32, crate::I32, crate::E64>,
    ),
    /// Add two unsigned 16-bit integer inputs and store the result into a vector register. No carry-in or carry-out support.
    VAddNcU16E64(
        crate::VectorBinary<family::VAddNc, crate::U16, crate::U16, crate::U16, crate::E64>,
    ),
    /// Add two unsigned 32-bit integer inputs and store the result into a vector register. No carry-in or carry-out support.
    VAddNcU32E32(
        crate::VectorBinary<family::VAddNc, crate::U32, crate::U32, crate::U32, crate::E32>,
    ),
    /// Add two unsigned 32-bit integer inputs and store the result into a vector register. No carry-in or carry-out support.
    VAddNcU32E64(
        crate::VectorBinary<family::VAddNc, crate::U32, crate::U32, crate::U32, crate::E64>,
    ),
    /// Align a 64-bit value encoded in the first two inputs to a bit position specified in the third input, then store the result into a 32-bit vector register.
    VAlignbitB32E64(
        crate::VectorTernary<
            family::VAlignbit,
            crate::B32,
            crate::B32,
            crate::B32,
            crate::U8,
            crate::E64,
        >,
    ),
    /// Align a 64-bit value encoded in the first two inputs to a byte position specified in the third input, then store the result into a 32-bit vector register.
    VAlignbyteB32E64(
        crate::VectorTernary<
            family::VAlignbyte,
            crate::B32,
            crate::B32,
            crate::B32,
            crate::U8,
            crate::E64,
        >,
    ),
    /// Calculate bitwise AND on two vector inputs and store the result into a vector register.
    VAndB32E32(crate::VectorBinary<family::VAnd, crate::B32, crate::B32, crate::B32, crate::E32>),
    /// Calculate bitwise AND on two vector inputs and store the result into a vector register.
    VAndB32E64(crate::VectorBinary<family::VAnd, crate::B32, crate::B32, crate::B32, crate::E64>),
    /// Calculate bitwise AND on the first two vector inputs, then compute the bitwise OR of the intermediate result and the third vector input, then store the final result into a vector register.
    VAndOrB32E64(
        crate::VectorTernary<
            family::VAndOr,
            crate::B32,
            crate::B32,
            crate::B32,
            crate::B32,
            crate::E64,
        >,
    ),
    /// Given a shift count in the first vector input, calculate the arithmetic shift right (preserving sign bit) of the second vector input and store the result into a vector register.
    VAshrrevI16E64(
        crate::VectorBinary<family::VAshrrev, crate::I16, crate::U16, crate::I16, crate::E64>,
    ),
    /// Given a shift count in the first vector input, calculate the arithmetic shift right (preserving sign bit) of the second vector input and store the result into a vector register.
    VAshrrevI32E32(
        crate::VectorBinary<family::VAshrrev, crate::I32, crate::U32, crate::I32, crate::E32>,
    ),
    /// Given a shift count in the first vector input, calculate the arithmetic shift right (preserving sign bit) of the second vector input and store the result into a vector register.
    VAshrrevI32E64(
        crate::VectorBinary<family::VAshrrev, crate::I32, crate::U32, crate::I32, crate::E64>,
    ),
    /// Given a shift count in the first vector input, calculate the arithmetic shift right (preserving sign bit) of the second vector input and store the result into a vector register.
    VAshrrevI64E64(
        crate::VectorBinary<family::VAshrrev, crate::I64, crate::U32, crate::I64, crate::E64>,
    ),
    /// Count the number of "1" bits in the vector input and store the result into a vector register.
    VBcntU32B32E64(
        crate::VectorBinary<family::VBcnt, crate::B32, crate::B32, crate::B32, crate::E64>,
    ),
    /// Extract a signed bitfield from the first input using field offset from the second input and size from the third input, then store the result into a vector register.
    VBfeI32E64(
        crate::VectorTernary<
            family::VBfe,
            crate::I32,
            crate::I32,
            crate::U32,
            crate::U32,
            crate::E64,
        >,
    ),
    /// Extract an unsigned bitfield from the first input using field offset from the second input and size from the third input, then store the result into a vector register.
    VBfeU32E64(
        crate::VectorTernary<
            family::VBfe,
            crate::U32,
            crate::U32,
            crate::U32,
            crate::U32,
            crate::E64,
        >,
    ),
    /// Overwrite a bitfield in the third input with a bitfield from the second input using a mask from the first input, then store the result into a vector register.
    VBfiB32E64(
        crate::VectorTernary<
            family::VBfi,
            crate::B32,
            crate::B32,
            crate::B32,
            crate::B32,
            crate::E64,
        >,
    ),
    /// Calculate a bitfield mask given a field offset and size and store the result into a vector register.
    VBfmB32E64(crate::VectorBinary<family::VBfm, crate::B32, crate::B32, crate::B32, crate::E64>),
    /// Reverse the order of bits in a vector input and store the result into a vector register.
    VBfrevB32E32(crate::VectorUnary<family::VBfrev, crate::B32, crate::B32, crate::E32>),
    /// Reverse the order of bits in a vector input and store the result into a vector register.
    VBfrevB32E64(crate::VectorUnary<family::VBfrev, crate::B32, crate::B32, crate::E64>),
    /// Round the half-precision float input up to next integer and store the result in floating point format into a vector register.
    VCeilF16E32(crate::VectorUnary<family::VCeil, crate::F16, crate::F16, crate::E32>),
    /// Round the half-precision float input up to next integer and store the result in floating point format into a vector register.
    VCeilF16E64(crate::VectorUnary<family::VCeil, crate::F16, crate::F16, crate::E64>),
    /// Round the single-precision float input up to next integer and store the result in floating point format into a vector register.
    VCeilF32E32(crate::VectorUnary<family::VCeil, crate::F32, crate::F32, crate::E32>),
    /// Round the single-precision float input up to next integer and store the result in floating point format into a vector register.
    VCeilF32E64(crate::VectorUnary<family::VCeil, crate::F32, crate::F32, crate::E64>),
    /// Round the double-precision float input up to next integer and store the result in floating point format into a vector register.
    VCeilF64E32(crate::VectorUnary<family::VCeil, crate::F64, crate::F64, crate::E32>),
    /// Round the double-precision float input up to next integer and store the result in floating point format into a vector register.
    VCeilF64E64(crate::VectorUnary<family::VCeil, crate::F64, crate::F64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is equal to the second input. Store the result into VCC or a scalar register.
    VCmpEqF16E32(crate::VCmp<predicate::Eq, crate::F16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is equal to the second input. Store the result into VCC or a scalar register.
    VCmpEqF16E64(crate::VCmp<predicate::Eq, crate::F16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is equal to the second input. Store the result into VCC or a scalar register.
    VCmpEqF32E32(crate::VCmp<predicate::Eq, crate::F32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is equal to the second input. Store the result into VCC or a scalar register.
    VCmpEqF32E64(crate::VCmp<predicate::Eq, crate::F32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is equal to the second input. Store the result into VCC or a scalar register.
    VCmpEqF64E32(crate::VCmp<predicate::Eq, crate::F64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is equal to the second input. Store the result into VCC or a scalar register.
    VCmpEqF64E64(crate::VCmp<predicate::Eq, crate::F64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is equal to the second input. Store the result into VCC or a scalar register.
    VCmpEqI16E32(crate::VCmp<predicate::Eq, crate::I16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is equal to the second input. Store the result into VCC or a scalar register.
    VCmpEqI16E64(crate::VCmp<predicate::Eq, crate::I16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is equal to the second input. Store the result into VCC or a scalar register.
    VCmpEqI32E32(crate::VCmp<predicate::Eq, crate::I32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is equal to the second input. Store the result into VCC or a scalar register.
    VCmpEqI32E64(crate::VCmp<predicate::Eq, crate::I32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is equal to the second input. Store the result into VCC or a scalar register.
    VCmpEqI64E32(crate::VCmp<predicate::Eq, crate::I64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is equal to the second input. Store the result into VCC or a scalar register.
    VCmpEqI64E64(crate::VCmp<predicate::Eq, crate::I64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is equal to the second input. Store the result into VCC or a scalar register.
    VCmpEqU16E32(crate::VCmp<predicate::Eq, crate::U16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is equal to the second input. Store the result into VCC or a scalar register.
    VCmpEqU16E64(crate::VCmp<predicate::Eq, crate::U16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is equal to the second input. Store the result into VCC or a scalar register.
    VCmpEqU32E32(crate::VCmp<predicate::Eq, crate::U32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is equal to the second input. Store the result into VCC or a scalar register.
    VCmpEqU32E64(crate::VCmp<predicate::Eq, crate::U32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is equal to the second input. Store the result into VCC or a scalar register.
    VCmpEqU64E32(crate::VCmp<predicate::Eq, crate::U64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is equal to the second input. Store the result into VCC or a scalar register.
    VCmpEqU64E64(crate::VCmp<predicate::Eq, crate::U64, crate::E64>),
    /// Set the per-lane condition code to 0. Store the result into VCC or a scalar register.
    VCmpFF16E32(crate::VCmp<predicate::Never, crate::F16, crate::E32>),
    /// Set the per-lane condition code to 0. Store the result into VCC or a scalar register.
    VCmpFF16E64(crate::VCmp<predicate::Never, crate::F16, crate::E64>),
    /// Set the per-lane condition code to 0. Store the result into VCC or a scalar register.
    VCmpFF32E32(crate::VCmp<predicate::Never, crate::F32, crate::E32>),
    /// Set the per-lane condition code to 0. Store the result into VCC or a scalar register.
    VCmpFF32E64(crate::VCmp<predicate::Never, crate::F32, crate::E64>),
    /// Set the per-lane condition code to 0. Store the result into VCC or a scalar register.
    VCmpFF64E32(crate::VCmp<predicate::Never, crate::F64, crate::E32>),
    /// Set the per-lane condition code to 0. Store the result into VCC or a scalar register.
    VCmpFF64E64(crate::VCmp<predicate::Never, crate::F64, crate::E64>),
    /// Set the per-lane condition code to 0. Store the result into VCC or a scalar register.
    VCmpFI32E32(crate::VCmp<predicate::Never, crate::I32, crate::E32>),
    /// Set the per-lane condition code to 0. Store the result into VCC or a scalar register.
    VCmpFI32E64(crate::VCmp<predicate::Never, crate::I32, crate::E64>),
    /// Set the per-lane condition code to 0. Store the result into VCC or a scalar register.
    VCmpFI64E32(crate::VCmp<predicate::Never, crate::I64, crate::E32>),
    /// Set the per-lane condition code to 0. Store the result into VCC or a scalar register.
    VCmpFI64E64(crate::VCmp<predicate::Never, crate::I64, crate::E64>),
    /// Set the per-lane condition code to 0. Store the result into VCC or a scalar register.
    VCmpFU32E32(crate::VCmp<predicate::Never, crate::U32, crate::E32>),
    /// Set the per-lane condition code to 0. Store the result into VCC or a scalar register.
    VCmpFU32E64(crate::VCmp<predicate::Never, crate::U32, crate::E64>),
    /// Set the per-lane condition code to 0. Store the result into VCC or a scalar register.
    VCmpFU64E32(crate::VCmp<predicate::Never, crate::U64, crate::E32>),
    /// Set the per-lane condition code to 0. Store the result into VCC or a scalar register.
    VCmpFU64E64(crate::VCmp<predicate::Never, crate::U64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpGeF16E32(crate::VCmp<predicate::Ge, crate::F16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpGeF16E64(crate::VCmp<predicate::Ge, crate::F16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpGeF32E32(crate::VCmp<predicate::Ge, crate::F32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpGeF32E64(crate::VCmp<predicate::Ge, crate::F32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpGeF64E32(crate::VCmp<predicate::Ge, crate::F64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpGeF64E64(crate::VCmp<predicate::Ge, crate::F64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpGeI16E32(crate::VCmp<predicate::Ge, crate::I16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpGeI16E64(crate::VCmp<predicate::Ge, crate::I16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpGeI32E32(crate::VCmp<predicate::Ge, crate::I32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpGeI32E64(crate::VCmp<predicate::Ge, crate::I32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpGeI64E32(crate::VCmp<predicate::Ge, crate::I64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpGeI64E64(crate::VCmp<predicate::Ge, crate::I64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpGeU16E32(crate::VCmp<predicate::Ge, crate::U16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpGeU16E64(crate::VCmp<predicate::Ge, crate::U16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpGeU32E32(crate::VCmp<predicate::Ge, crate::U32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpGeU32E64(crate::VCmp<predicate::Ge, crate::U32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpGeU64E32(crate::VCmp<predicate::Ge, crate::U64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpGeU64E64(crate::VCmp<predicate::Ge, crate::U64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is greater than the second input. Store the result into VCC or a scalar register.
    VCmpGtF16E32(crate::VCmp<predicate::Gt, crate::F16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is greater than the second input. Store the result into VCC or a scalar register.
    VCmpGtF16E64(crate::VCmp<predicate::Gt, crate::F16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is greater than the second input. Store the result into VCC or a scalar register.
    VCmpGtF32E32(crate::VCmp<predicate::Gt, crate::F32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is greater than the second input. Store the result into VCC or a scalar register.
    VCmpGtF32E64(crate::VCmp<predicate::Gt, crate::F32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is greater than the second input. Store the result into VCC or a scalar register.
    VCmpGtF64E32(crate::VCmp<predicate::Gt, crate::F64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is greater than the second input. Store the result into VCC or a scalar register.
    VCmpGtF64E64(crate::VCmp<predicate::Gt, crate::F64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is greater than the second input. Store the result into VCC or a scalar register.
    VCmpGtI16E32(crate::VCmp<predicate::Gt, crate::I16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is greater than the second input. Store the result into VCC or a scalar register.
    VCmpGtI16E64(crate::VCmp<predicate::Gt, crate::I16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is greater than the second input. Store the result into VCC or a scalar register.
    VCmpGtI32E32(crate::VCmp<predicate::Gt, crate::I32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is greater than the second input. Store the result into VCC or a scalar register.
    VCmpGtI32E64(crate::VCmp<predicate::Gt, crate::I32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is greater than the second input. Store the result into VCC or a scalar register.
    VCmpGtI64E32(crate::VCmp<predicate::Gt, crate::I64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is greater than the second input. Store the result into VCC or a scalar register.
    VCmpGtI64E64(crate::VCmp<predicate::Gt, crate::I64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is greater than the second input. Store the result into VCC or a scalar register.
    VCmpGtU16E32(crate::VCmp<predicate::Gt, crate::U16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is greater than the second input. Store the result into VCC or a scalar register.
    VCmpGtU16E64(crate::VCmp<predicate::Gt, crate::U16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is greater than the second input. Store the result into VCC or a scalar register.
    VCmpGtU32E32(crate::VCmp<predicate::Gt, crate::U32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is greater than the second input. Store the result into VCC or a scalar register.
    VCmpGtU32E64(crate::VCmp<predicate::Gt, crate::U32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is greater than the second input. Store the result into VCC or a scalar register.
    VCmpGtU64E32(crate::VCmp<predicate::Gt, crate::U64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is greater than the second input. Store the result into VCC or a scalar register.
    VCmpGtU64E64(crate::VCmp<predicate::Gt, crate::U64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpLeF16E32(crate::VCmp<predicate::Le, crate::F16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpLeF16E64(crate::VCmp<predicate::Le, crate::F16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpLeF32E32(crate::VCmp<predicate::Le, crate::F32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpLeF32E64(crate::VCmp<predicate::Le, crate::F32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpLeF64E32(crate::VCmp<predicate::Le, crate::F64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpLeF64E64(crate::VCmp<predicate::Le, crate::F64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpLeI16E32(crate::VCmp<predicate::Le, crate::I16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpLeI16E64(crate::VCmp<predicate::Le, crate::I16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpLeI32E32(crate::VCmp<predicate::Le, crate::I32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpLeI32E64(crate::VCmp<predicate::Le, crate::I32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpLeI64E32(crate::VCmp<predicate::Le, crate::I64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpLeI64E64(crate::VCmp<predicate::Le, crate::I64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpLeU16E32(crate::VCmp<predicate::Le, crate::U16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpLeU16E64(crate::VCmp<predicate::Le, crate::U16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpLeU32E32(crate::VCmp<predicate::Le, crate::U32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpLeU32E64(crate::VCmp<predicate::Le, crate::U32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpLeU64E32(crate::VCmp<predicate::Le, crate::U64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpLeU64E64(crate::VCmp<predicate::Le, crate::U64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is less than or greater than the second input. Store the result into VCC or a scalar register.
    VCmpLgF16E32(crate::VCmp<predicate::Lg, crate::F16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is less than or greater than the second input. Store the result into VCC or a scalar register.
    VCmpLgF16E64(crate::VCmp<predicate::Lg, crate::F16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is less than or greater than the second input. Store the result into VCC or a scalar register.
    VCmpLgF32E32(crate::VCmp<predicate::Lg, crate::F32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is less than or greater than the second input. Store the result into VCC or a scalar register.
    VCmpLgF32E64(crate::VCmp<predicate::Lg, crate::F32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is less than or greater than the second input. Store the result into VCC or a scalar register.
    VCmpLgF64E32(crate::VCmp<predicate::Lg, crate::F64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is less than or greater than the second input. Store the result into VCC or a scalar register.
    VCmpLgF64E64(crate::VCmp<predicate::Lg, crate::F64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is less than the second input. Store the result into VCC or a scalar register.
    VCmpLtF16E32(crate::VCmp<predicate::Lt, crate::F16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is less than the second input. Store the result into VCC or a scalar register.
    VCmpLtF16E64(crate::VCmp<predicate::Lt, crate::F16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is less than the second input. Store the result into VCC or a scalar register.
    VCmpLtF32E32(crate::VCmp<predicate::Lt, crate::F32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is less than the second input. Store the result into VCC or a scalar register.
    VCmpLtF32E64(crate::VCmp<predicate::Lt, crate::F32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is less than the second input. Store the result into VCC or a scalar register.
    VCmpLtF64E32(crate::VCmp<predicate::Lt, crate::F64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is less than the second input. Store the result into VCC or a scalar register.
    VCmpLtF64E64(crate::VCmp<predicate::Lt, crate::F64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is less than the second input. Store the result into VCC or a scalar register.
    VCmpLtI16E32(crate::VCmp<predicate::Lt, crate::I16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is less than the second input. Store the result into VCC or a scalar register.
    VCmpLtI16E64(crate::VCmp<predicate::Lt, crate::I16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is less than the second input. Store the result into VCC or a scalar register.
    VCmpLtI32E32(crate::VCmp<predicate::Lt, crate::I32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is less than the second input. Store the result into VCC or a scalar register.
    VCmpLtI32E64(crate::VCmp<predicate::Lt, crate::I32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is less than the second input. Store the result into VCC or a scalar register.
    VCmpLtI64E32(crate::VCmp<predicate::Lt, crate::I64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is less than the second input. Store the result into VCC or a scalar register.
    VCmpLtI64E64(crate::VCmp<predicate::Lt, crate::I64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is less than the second input. Store the result into VCC or a scalar register.
    VCmpLtU16E32(crate::VCmp<predicate::Lt, crate::U16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is less than the second input. Store the result into VCC or a scalar register.
    VCmpLtU16E64(crate::VCmp<predicate::Lt, crate::U16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is less than the second input. Store the result into VCC or a scalar register.
    VCmpLtU32E32(crate::VCmp<predicate::Lt, crate::U32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is less than the second input. Store the result into VCC or a scalar register.
    VCmpLtU32E64(crate::VCmp<predicate::Lt, crate::U32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is less than the second input. Store the result into VCC or a scalar register.
    VCmpLtU64E32(crate::VCmp<predicate::Lt, crate::U64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is less than the second input. Store the result into VCC or a scalar register.
    VCmpLtU64E64(crate::VCmp<predicate::Lt, crate::U64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not equal to the second input. Store the result into VCC or a scalar register.
    VCmpNeI16E32(crate::VCmp<predicate::Ne, crate::I16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not equal to the second input. Store the result into VCC or a scalar register.
    VCmpNeI16E64(crate::VCmp<predicate::Ne, crate::I16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not equal to the second input. Store the result into VCC or a scalar register.
    VCmpNeI32E32(crate::VCmp<predicate::Ne, crate::I32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not equal to the second input. Store the result into VCC or a scalar register.
    VCmpNeI32E64(crate::VCmp<predicate::Ne, crate::I32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not equal to the second input. Store the result into VCC or a scalar register.
    VCmpNeI64E32(crate::VCmp<predicate::Ne, crate::I64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not equal to the second input. Store the result into VCC or a scalar register.
    VCmpNeI64E64(crate::VCmp<predicate::Ne, crate::I64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not equal to the second input. Store the result into VCC or a scalar register.
    VCmpNeU16E32(crate::VCmp<predicate::Ne, crate::U16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not equal to the second input. Store the result into VCC or a scalar register.
    VCmpNeU16E64(crate::VCmp<predicate::Ne, crate::U16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not equal to the second input. Store the result into VCC or a scalar register.
    VCmpNeU32E32(crate::VCmp<predicate::Ne, crate::U32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not equal to the second input. Store the result into VCC or a scalar register.
    VCmpNeU32E64(crate::VCmp<predicate::Ne, crate::U32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not equal to the second input. Store the result into VCC or a scalar register.
    VCmpNeU64E32(crate::VCmp<predicate::Ne, crate::U64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not equal to the second input. Store the result into VCC or a scalar register.
    VCmpNeU64E64(crate::VCmp<predicate::Ne, crate::U64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpNgeF16E32(crate::VCmp<predicate::NotGe, crate::F16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpNgeF16E64(crate::VCmp<predicate::NotGe, crate::F16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpNgeF32E32(crate::VCmp<predicate::NotGe, crate::F32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpNgeF32E64(crate::VCmp<predicate::NotGe, crate::F32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpNgeF64E32(crate::VCmp<predicate::NotGe, crate::F64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not greater than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpNgeF64E64(crate::VCmp<predicate::NotGe, crate::F64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not greater than the second input. Store the result into VCC or a scalar register.
    VCmpNgtF16E32(crate::VCmp<predicate::NotGt, crate::F16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not greater than the second input. Store the result into VCC or a scalar register.
    VCmpNgtF16E64(crate::VCmp<predicate::NotGt, crate::F16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not greater than the second input. Store the result into VCC or a scalar register.
    VCmpNgtF32E32(crate::VCmp<predicate::NotGt, crate::F32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not greater than the second input. Store the result into VCC or a scalar register.
    VCmpNgtF32E64(crate::VCmp<predicate::NotGt, crate::F32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not greater than the second input. Store the result into VCC or a scalar register.
    VCmpNgtF64E32(crate::VCmp<predicate::NotGt, crate::F64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not greater than the second input. Store the result into VCC or a scalar register.
    VCmpNgtF64E64(crate::VCmp<predicate::NotGt, crate::F64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpNleF16E32(crate::VCmp<predicate::NotLe, crate::F16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpNleF16E64(crate::VCmp<predicate::NotLe, crate::F16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpNleF32E32(crate::VCmp<predicate::NotLe, crate::F32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpNleF32E64(crate::VCmp<predicate::NotLe, crate::F32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpNleF64E32(crate::VCmp<predicate::NotLe, crate::F64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not less than or equal to the second input. Store the result into VCC or a scalar register.
    VCmpNleF64E64(crate::VCmp<predicate::NotLe, crate::F64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not less than or greater than the second input. Store the result into VCC or a scalar register.
    VCmpNlgF16E32(crate::VCmp<predicate::NotLg, crate::F16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not less than or greater than the second input. Store the result into VCC or a scalar register.
    VCmpNlgF16E64(crate::VCmp<predicate::NotLg, crate::F16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not less than or greater than the second input. Store the result into VCC or a scalar register.
    VCmpNlgF32E32(crate::VCmp<predicate::NotLg, crate::F32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not less than or greater than the second input. Store the result into VCC or a scalar register.
    VCmpNlgF32E64(crate::VCmp<predicate::NotLg, crate::F32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not less than or greater than the second input. Store the result into VCC or a scalar register.
    VCmpNlgF64E32(crate::VCmp<predicate::NotLg, crate::F64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not less than or greater than the second input. Store the result into VCC or a scalar register.
    VCmpNlgF64E64(crate::VCmp<predicate::NotLg, crate::F64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not less than the second input. Store the result into VCC or a scalar register.
    VCmpNltF16E32(crate::VCmp<predicate::NotLt, crate::F16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not less than the second input. Store the result into VCC or a scalar register.
    VCmpNltF16E64(crate::VCmp<predicate::NotLt, crate::F16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not less than the second input. Store the result into VCC or a scalar register.
    VCmpNltF32E32(crate::VCmp<predicate::NotLt, crate::F32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not less than the second input. Store the result into VCC or a scalar register.
    VCmpNltF32E64(crate::VCmp<predicate::NotLt, crate::F32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not less than the second input. Store the result into VCC or a scalar register.
    VCmpNltF64E32(crate::VCmp<predicate::NotLt, crate::F64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not less than the second input. Store the result into VCC or a scalar register.
    VCmpNltF64E64(crate::VCmp<predicate::NotLt, crate::F64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is orderable to the second input. Store the result into VCC or a scalar register.
    VCmpOF16E32(crate::VCmp<predicate::Ordered, crate::F16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is orderable to the second input. Store the result into VCC or a scalar register.
    VCmpOF16E64(crate::VCmp<predicate::Ordered, crate::F16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is orderable to the second input. Store the result into VCC or a scalar register.
    VCmpOF32E32(crate::VCmp<predicate::Ordered, crate::F32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is orderable to the second input. Store the result into VCC or a scalar register.
    VCmpOF32E64(crate::VCmp<predicate::Ordered, crate::F32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is orderable to the second input. Store the result into VCC or a scalar register.
    VCmpOF64E32(crate::VCmp<predicate::Ordered, crate::F64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is orderable to the second input. Store the result into VCC or a scalar register.
    VCmpOF64E64(crate::VCmp<predicate::Ordered, crate::F64, crate::E64>),
    /// Set the per-lane condition code to 1. Store the result into VCC or a scalar register.
    VCmpTI32E32(crate::VCmp<predicate::Always, crate::I32, crate::E32>),
    /// Set the per-lane condition code to 1. Store the result into VCC or a scalar register.
    VCmpTI32E64(crate::VCmp<predicate::Always, crate::I32, crate::E64>),
    /// Set the per-lane condition code to 1. Store the result into VCC or a scalar register.
    VCmpTI64E32(crate::VCmp<predicate::Always, crate::I64, crate::E32>),
    /// Set the per-lane condition code to 1. Store the result into VCC or a scalar register.
    VCmpTI64E64(crate::VCmp<predicate::Always, crate::I64, crate::E64>),
    /// Set the per-lane condition code to 1. Store the result into VCC or a scalar register.
    VCmpTU32E32(crate::VCmp<predicate::Always, crate::U32, crate::E32>),
    /// Set the per-lane condition code to 1. Store the result into VCC or a scalar register.
    VCmpTU32E64(crate::VCmp<predicate::Always, crate::U32, crate::E64>),
    /// Set the per-lane condition code to 1. Store the result into VCC or a scalar register.
    VCmpTU64E32(crate::VCmp<predicate::Always, crate::U64, crate::E32>),
    /// Set the per-lane condition code to 1. Store the result into VCC or a scalar register.
    VCmpTU64E64(crate::VCmp<predicate::Always, crate::U64, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not orderable to the second input. Store the result into VCC or a scalar register.
    VCmpUF16E32(crate::VCmp<predicate::Unordered, crate::F16, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not orderable to the second input. Store the result into VCC or a scalar register.
    VCmpUF16E64(crate::VCmp<predicate::Unordered, crate::F16, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not orderable to the second input. Store the result into VCC or a scalar register.
    VCmpUF32E32(crate::VCmp<predicate::Unordered, crate::F32, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not orderable to the second input. Store the result into VCC or a scalar register.
    VCmpUF32E64(crate::VCmp<predicate::Unordered, crate::F32, crate::E64>),
    /// Set the per-lane condition code to 1 iff the first input is not orderable to the second input. Store the result into VCC or a scalar register.
    VCmpUF64E32(crate::VCmp<predicate::Unordered, crate::F64, crate::E32>),
    /// Set the per-lane condition code to 1 iff the first input is not orderable to the second input. Store the result into VCC or a scalar register.
    VCmpUF64E64(crate::VCmp<predicate::Unordered, crate::F64, crate::E64>),
    /// Calculate the trigonometric cosine of a half-precision float value using IEEE rules and store the result into a vector register. The operand is calculated by scaling the vector input by 2 PI.
    VCosF16E32(crate::VectorUnary<family::VCos, crate::F16, crate::F16, crate::E32>),
    /// Calculate the trigonometric cosine of a half-precision float value using IEEE rules and store the result into a vector register. The operand is calculated by scaling the vector input by 2 PI.
    VCosF16E64(crate::VectorUnary<family::VCos, crate::F16, crate::F16, crate::E64>),
    /// Calculate the trigonometric cosine of a single-precision float value using IEEE rules and store the result into a vector register. The operand is calculated by scaling the vector input by 2 PI.
    VCosF32E32(crate::VectorUnary<family::VCos, crate::F32, crate::F32, crate::E32>),
    /// Calculate the trigonometric cosine of a single-precision float value using IEEE rules and store the result into a vector register. The operand is calculated by scaling the vector input by 2 PI.
    VCosF32E64(crate::VectorUnary<family::VCos, crate::F32, crate::F32, crate::E64>),
    /// Compute the cubemap face ID of a 3D coordinate specified as three single-precision float inputs. Store the result in single-precision float format into a vector register.
    VCubeidF32E64(
        crate::VectorTernary<
            family::VCubeid,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::E64,
        >,
    ),
    /// Compute the cubemap major axis of a 3D coordinate specified as three single-precision float inputs. Store the result in single-precision float format into a vector register.
    VCubemaF32E64(
        crate::VectorTernary<
            family::VCubema,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::E64,
        >,
    ),
    /// Compute the cubemap S coordinate of a 3D coordinate specified as three single-precision float inputs. Store the result in single-precision float format into a vector register.
    VCubescF32E64(
        crate::VectorTernary<
            family::VCubesc,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::E64,
        >,
    ),
    /// Compute the cubemap T coordinate of a 3D coordinate specified as three single-precision float inputs. Store the result in single-precision float format into a vector register.
    VCubetcF32E64(
        crate::VectorTernary<
            family::VCubetc,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::E64,
        >,
    ),
    /// Convert from a single-precision float input to a half-precision float value and store the result into a vector register.
    VCvtF16F32E32(crate::VectorUnary<family::VCvt, crate::F16, crate::F32, crate::E32>),
    /// Convert from a single-precision float input to a half-precision float value and store the result into a vector register.
    VCvtF16F32E64(crate::VectorUnary<family::VCvt, crate::F16, crate::F32, crate::E64>),
    /// Convert from a signed 16-bit integer input to a half-precision float value and store the result into a vector register.
    VCvtF16I16E32(crate::VectorUnary<family::VCvt, crate::F16, crate::I16, crate::E32>),
    /// Convert from a signed 16-bit integer input to a half-precision float value and store the result into a vector register.
    VCvtF16I16E64(crate::VectorUnary<family::VCvt, crate::F16, crate::I16, crate::E64>),
    /// Convert from an unsigned 16-bit integer input to a half-precision float value and store the result into a vector register.
    VCvtF16U16E32(crate::VectorUnary<family::VCvt, crate::F16, crate::U16, crate::E32>),
    /// Convert from an unsigned 16-bit integer input to a half-precision float value and store the result into a vector register.
    VCvtF16U16E64(crate::VectorUnary<family::VCvt, crate::F16, crate::U16, crate::E64>),
    /// Convert from a half-precision float input to a single-precision float value and store the result into a vector register.
    VCvtF32F16E32(crate::VectorUnary<family::VCvt, crate::F32, crate::F16, crate::E32>),
    /// Convert from a half-precision float input to a single-precision float value and store the result into a vector register.
    VCvtF32F16E64(crate::VectorUnary<family::VCvt, crate::F32, crate::F16, crate::E64>),
    /// Convert from a double-precision float input to a single-precision float value and store the result into a vector register.
    VCvtF32F64E32(crate::VectorUnary<family::VCvt, crate::F32, crate::F64, crate::E32>),
    /// Convert from a double-precision float input to a single-precision float value and store the result into a vector register.
    VCvtF32F64E64(crate::VectorUnary<family::VCvt, crate::F32, crate::F64, crate::E64>),
    /// Convert from a signed 32-bit integer input to a single-precision float value and store the result into a vector register.
    VCvtF32I32E32(crate::VectorUnary<family::VCvt, crate::F32, crate::I32, crate::E32>),
    /// Convert from a signed 32-bit integer input to a single-precision float value and store the result into a vector register.
    VCvtF32I32E64(crate::VectorUnary<family::VCvt, crate::F32, crate::I32, crate::E64>),
    /// Convert from an unsigned 32-bit integer input to a single-precision float value and store the result into a vector register.
    VCvtF32U32E32(crate::VectorUnary<family::VCvt, crate::F32, crate::U32, crate::E32>),
    /// Convert from an unsigned 32-bit integer input to a single-precision float value and store the result into a vector register.
    VCvtF32U32E64(crate::VectorUnary<family::VCvt, crate::F32, crate::U32, crate::E64>),
    /// Convert from a single-precision float input to a double-precision float value and store the result into a vector register.
    VCvtF64F32E32(crate::VectorUnary<family::VCvt, crate::F64, crate::F32, crate::E32>),
    /// Convert from a single-precision float input to a double-precision float value and store the result into a vector register.
    VCvtF64F32E64(crate::VectorUnary<family::VCvt, crate::F64, crate::F32, crate::E64>),
    /// Convert from a signed 32-bit integer input to a double-precision float value and store the result into a vector register.
    VCvtF64I32E32(crate::VectorUnary<family::VCvt, crate::F64, crate::I32, crate::E32>),
    /// Convert from a signed 32-bit integer input to a double-precision float value and store the result into a vector register.
    VCvtF64I32E64(crate::VectorUnary<family::VCvt, crate::F64, crate::I32, crate::E64>),
    /// Convert from an unsigned 32-bit integer input to a double-precision float value and store the result into a vector register.
    VCvtF64U32E32(crate::VectorUnary<family::VCvt, crate::F64, crate::U32, crate::E32>),
    /// Convert from an unsigned 32-bit integer input to a double-precision float value and store the result into a vector register.
    VCvtF64U32E64(crate::VectorUnary<family::VCvt, crate::F64, crate::U32, crate::E64>),
    /// Convert from a single-precision float input to a signed 32-bit integer value using round-down semantics (ignore the default rounding mode) and store the result into a vector register.
    VCvtFlrI32F32E32(crate::VectorUnary<family::VCvtFlr, crate::I32, crate::F32, crate::E32>),
    /// Convert from a single-precision float input to a signed 32-bit integer value using round-down semantics (ignore the default rounding mode) and store the result into a vector register.
    VCvtFlrI32F32E64(crate::VectorUnary<family::VCvtFlr, crate::I32, crate::F32, crate::E64>),
    /// Convert from a half-precision float input to a signed 16-bit integer value and store the result into a vector register.
    VCvtI16F16E32(crate::VectorUnary<family::VCvt, crate::I16, crate::F16, crate::E32>),
    /// Convert from a half-precision float input to a signed 16-bit integer value and store the result into a vector register.
    VCvtI16F16E64(crate::VectorUnary<family::VCvt, crate::I16, crate::F16, crate::E64>),
    /// Convert from a single-precision float input to a signed 32-bit integer value and store the result into a vector register.
    VCvtI32F32E32(crate::VectorUnary<family::VCvt, crate::I32, crate::F32, crate::E32>),
    /// Convert from a single-precision float input to a signed 32-bit integer value and store the result into a vector register.
    VCvtI32F32E64(crate::VectorUnary<family::VCvt, crate::I32, crate::F32, crate::E64>),
    /// Convert from a double-precision float input to a signed 32-bit integer value and store the result into a vector register.
    VCvtI32F64E32(crate::VectorUnary<family::VCvt, crate::I32, crate::F64, crate::E32>),
    /// Convert from a double-precision float input to a signed 32-bit integer value and store the result into a vector register.
    VCvtI32F64E64(crate::VectorUnary<family::VCvt, crate::I32, crate::F64, crate::E64>),
    /// Convert from a half-precision float input to a signed normalized short and store the result into a vector register.
    VCvtNormI16F16E32(crate::VectorUnary<family::VCvtNorm, crate::I16, crate::F16, crate::E32>),
    /// Convert from a half-precision float input to a signed normalized short and store the result into a vector register.
    VCvtNormI16F16E64(crate::VectorUnary<family::VCvtNorm, crate::I16, crate::F16, crate::E64>),
    /// Convert from a half-precision float input to an unsigned normalized short and store the result into a vector register.
    VCvtNormU16F16E32(crate::VectorUnary<family::VCvtNorm, crate::U16, crate::F16, crate::E32>),
    /// Convert from a half-precision float input to an unsigned normalized short and store the result into a vector register.
    VCvtNormU16F16E64(crate::VectorUnary<family::VCvtNorm, crate::U16, crate::F16, crate::E64>),
    /// Convert from a single-precision float input to a signed 32-bit integer value using round to nearest integer semantics (ignore the default rounding mode) and store the result into a vector register.
    VCvtRpiI32F32E32(crate::VectorUnary<family::VCvtRpi, crate::I32, crate::F32, crate::E32>),
    /// Convert from a single-precision float input to a signed 32-bit integer value using round to nearest integer semantics (ignore the default rounding mode) and store the result into a vector register.
    VCvtRpiI32F32E64(crate::VectorUnary<family::VCvtRpi, crate::I32, crate::F32, crate::E64>),
    /// Convert from a half-precision float input to an unsigned 16-bit integer value and store the result into a vector register.
    VCvtU16F16E32(crate::VectorUnary<family::VCvt, crate::U16, crate::F16, crate::E32>),
    /// Convert from a half-precision float input to an unsigned 16-bit integer value and store the result into a vector register.
    VCvtU16F16E64(crate::VectorUnary<family::VCvt, crate::U16, crate::F16, crate::E64>),
    /// Convert from a single-precision float input to an unsigned 32-bit integer value and store the result into a vector register.
    VCvtU32F32E32(crate::VectorUnary<family::VCvt, crate::U32, crate::F32, crate::E32>),
    /// Convert from a single-precision float input to an unsigned 32-bit integer value and store the result into a vector register.
    VCvtU32F32E64(crate::VectorUnary<family::VCvt, crate::U32, crate::F32, crate::E64>),
    /// Convert from a double-precision float input to an unsigned 32-bit integer value and store the result into a vector register.
    VCvtU32F64E32(crate::VectorUnary<family::VCvt, crate::U32, crate::F64, crate::E32>),
    /// Convert from a double-precision float input to an unsigned 32-bit integer value and store the result into a vector register.
    VCvtU32F64E64(crate::VectorUnary<family::VCvt, crate::U32, crate::F64, crate::E64>),
    /// Given a half-precision float quotient in the first input, a denominator in the second input and a numerator in the third input, detect and apply corner cases related to division, including divide by zero, NaN inputs and overflow, and modify the quotient accordingly. Generate any invalid, denormal and divide-by-zero exceptions that are a result of the division. Store the modified quotient into a vector register.
    VDivFixupF16E64(
        crate::VectorTernary<
            family::VDivFixup,
            crate::F16,
            crate::F16,
            crate::F16,
            crate::F16,
            crate::E64,
        >,
    ),
    /// Given a single-precision float quotient in the first input, a denominator in the second input and a numerator in the third input, detect and apply corner cases related to division, including divide by zero, NaN inputs and overflow, and modify the quotient accordingly. Generate any invalid, denormal and divide-by-zero exceptions that are a result of the division. Store the modified quotient into a vector register.
    VDivFixupF32E64(
        crate::VectorTernary<
            family::VDivFixup,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::E64,
        >,
    ),
    /// Given a double-precision float quotient in the first input, a denominator in the second input and a numerator in the third input, detect and apply corner cases related to division, including divide by zero, NaN inputs and overflow, and modify the quotient accordingly. Generate any invalid, denormal and divide-by-zero exceptions that are a result of the division. Store the modified quotient into a vector register.
    VDivFixupF64E64(
        crate::VectorTernary<
            family::VDivFixup,
            crate::F64,
            crate::F64,
            crate::F64,
            crate::F64,
            crate::E64,
        >,
    ),
    /// Calculate 2 raised to the power of the half-precision float input and store the result into a vector register.
    VExpF16E32(crate::VectorUnary<family::VExp, crate::F16, crate::F16, crate::E32>),
    /// Calculate 2 raised to the power of the half-precision float input and store the result into a vector register.
    VExpF16E64(crate::VectorUnary<family::VExp, crate::F16, crate::F16, crate::E64>),
    /// Calculate 2 raised to the power of the single-precision float input and store the result into a vector register.
    VExpF32E32(crate::VectorUnary<family::VExp, crate::F32, crate::F32, crate::E32>),
    /// Calculate 2 raised to the power of the single-precision float input and store the result into a vector register.
    VExpF32E64(crate::VectorUnary<family::VExp, crate::F32, crate::F32, crate::E64>),
    /// Count the number of leading bits that are the same as the sign bit of a vector input and store the result into a vector register. Store -1 if all input bits are the same.
    VFfbhI32E32(crate::VectorUnary<family::VFfbh, crate::I32, crate::I32, crate::E32>),
    /// Count the number of leading bits that are the same as the sign bit of a vector input and store the result into a vector register. Store -1 if all input bits are the same.
    VFfbhI32E64(crate::VectorUnary<family::VFfbh, crate::I32, crate::I32, crate::E64>),
    /// Count the number of leading "0" bits before the first "1" in a vector input and store the result into a vector register. Store -1 if there are no "1" bits.
    VFfbhU32E32(crate::VectorUnary<family::VFfbh, crate::I32, crate::U32, crate::E32>),
    /// Count the number of leading "0" bits before the first "1" in a vector input and store the result into a vector register. Store -1 if there are no "1" bits.
    VFfbhU32E64(crate::VectorUnary<family::VFfbh, crate::I32, crate::U32, crate::E64>),
    /// Count the number of trailing "0" bits before the first "1" in a vector input and store the result into a vector register. Store -1 if there are no "1" bits in the input.
    VFfblB32E32(crate::VectorUnary<family::VFfbl, crate::B32, crate::B32, crate::E32>),
    /// Count the number of trailing "0" bits before the first "1" in a vector input and store the result into a vector register. Store -1 if there are no "1" bits in the input.
    VFfblB32E64(crate::VectorUnary<family::VFfbl, crate::B32, crate::B32, crate::E64>),
    /// Round the half-precision float input down to previous integer and store the result in floating point format into a vector register.
    VFloorF16E32(crate::VectorUnary<family::VFloor, crate::F16, crate::F16, crate::E32>),
    /// Round the half-precision float input down to previous integer and store the result in floating point format into a vector register.
    VFloorF16E64(crate::VectorUnary<family::VFloor, crate::F16, crate::F16, crate::E64>),
    /// Round the single-precision float input down to previous integer and store the result in floating point format into a vector register.
    VFloorF32E32(crate::VectorUnary<family::VFloor, crate::F32, crate::F32, crate::E32>),
    /// Round the single-precision float input down to previous integer and store the result in floating point format into a vector register.
    VFloorF32E64(crate::VectorUnary<family::VFloor, crate::F32, crate::F32, crate::E64>),
    /// Round the double-precision float input down to previous integer and store the result in floating point format into a vector register.
    VFloorF64E32(crate::VectorUnary<family::VFloor, crate::F64, crate::F64, crate::E32>),
    /// Round the double-precision float input down to previous integer and store the result in floating point format into a vector register.
    VFloorF64E64(crate::VectorUnary<family::VFloor, crate::F64, crate::F64, crate::E64>),
    /// Multiply two half-precision float inputs and add a third input using fused multiply add, and store the result into a vector register.
    VFmaF16E64(
        crate::VectorTernary<
            family::VFma,
            crate::F16,
            crate::F16,
            crate::F16,
            crate::F16,
            crate::E64,
        >,
    ),
    /// Multiply two single-precision float inputs and add a third input using fused multiply add, and store the result into a vector register.
    VFmaF32E64(
        crate::VectorTernary<
            family::VFma,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::E64,
        >,
    ),
    /// Multiply two double-precision float inputs and add a third input using fused multiply add, and store the result into a vector register.
    VFmaF64E64(
        crate::VectorTernary<
            family::VFma,
            crate::F64,
            crate::F64,
            crate::F64,
            crate::F64,
            crate::E64,
        >,
    ),
    /// Multiply and add single-precision values. Follows DX9 rules where 0.0 times anything produces 0.0.
    VFmaLegacyF32E64(
        crate::VectorTernary<
            family::VFmaLegacy,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::E64,
        >,
    ),
    /// Multiply two half-precision float inputs and accumulate the result into the destination register using fused multiply add.
    VFmacF16E32(crate::VectorBinary<family::VFmac, crate::F16, crate::F16, crate::F16, crate::E32>),
    /// Multiply two half-precision float inputs and accumulate the result into the destination register using fused multiply add.
    VFmacF16E64(crate::VectorBinary<family::VFmac, crate::F16, crate::F16, crate::F16, crate::E64>),
    /// Multiply two single-precision float inputs and accumulate the result into the destination register using fused multiply add.
    VFmacF32E32(crate::VectorBinary<family::VFmac, crate::F32, crate::F32, crate::F32, crate::E32>),
    /// Multiply two single-precision float inputs and accumulate the result into the destination register using fused multiply add.
    VFmacF32E64(crate::VectorBinary<family::VFmac, crate::F32, crate::F32, crate::F32, crate::E64>),
    /// Multiply two single-precision values and accumulate the result with the destination. Follows DX9 rules where 0.0 times anything produces 0.0.
    VFmacLegacyF32E32(
        crate::VectorBinary<family::VFmacLegacy, crate::F32, crate::F32, crate::F32, crate::E32>,
    ),
    /// Multiply two single-precision values and accumulate the result with the destination. Follows DX9 rules where 0.0 times anything produces 0.0.
    VFmacLegacyF32E64(
        crate::VectorBinary<family::VFmacLegacy, crate::F32, crate::F32, crate::F32, crate::E64>,
    ),
    /// Compute the fractional portion of a half-precision float input and store the result in floating point format into a vector register.
    VFractF16E32(crate::VectorUnary<family::VFract, crate::F16, crate::F16, crate::E32>),
    /// Compute the fractional portion of a half-precision float input and store the result in floating point format into a vector register.
    VFractF16E64(crate::VectorUnary<family::VFract, crate::F16, crate::F16, crate::E64>),
    /// Compute the fractional portion of a single-precision float input and store the result in floating point format into a vector register.
    VFractF32E32(crate::VectorUnary<family::VFract, crate::F32, crate::F32, crate::E32>),
    /// Compute the fractional portion of a single-precision float input and store the result in floating point format into a vector register.
    VFractF32E64(crate::VectorUnary<family::VFract, crate::F32, crate::F32, crate::E64>),
    /// Compute the fractional portion of a double-precision float input and store the result in floating point format into a vector register.
    VFractF64E32(crate::VectorUnary<family::VFract, crate::F64, crate::F64, crate::E32>),
    /// Compute the fractional portion of a double-precision float input and store the result in floating point format into a vector register.
    VFractF64E64(crate::VectorUnary<family::VFract, crate::F64, crate::F64, crate::E64>),
    /// Extract the exponent of a half-precision float input and store the result as a signed 16-bit integer into a vector register.
    VFrexpExpI16F16E32(crate::VectorUnary<family::VFrexpExp, crate::I16, crate::F16, crate::E32>),
    /// Extract the exponent of a half-precision float input and store the result as a signed 16-bit integer into a vector register.
    VFrexpExpI16F16E64(crate::VectorUnary<family::VFrexpExp, crate::I16, crate::F16, crate::E64>),
    /// Extract the exponent of a single-precision float input and store the result as a signed 32-bit integer into a vector register.
    VFrexpExpI32F32E32(crate::VectorUnary<family::VFrexpExp, crate::I32, crate::F32, crate::E32>),
    /// Extract the exponent of a single-precision float input and store the result as a signed 32-bit integer into a vector register.
    VFrexpExpI32F32E64(crate::VectorUnary<family::VFrexpExp, crate::I32, crate::F32, crate::E64>),
    /// Extract the exponent of a double-precision float input and store the result as a signed 32-bit integer into a vector register.
    VFrexpExpI32F64E32(crate::VectorUnary<family::VFrexpExp, crate::I32, crate::F64, crate::E32>),
    /// Extract the exponent of a double-precision float input and store the result as a signed 32-bit integer into a vector register.
    VFrexpExpI32F64E64(crate::VectorUnary<family::VFrexpExp, crate::I32, crate::F64, crate::E64>),
    /// Extract the binary significand, or mantissa, of a half-precision float input and store the result as a half-precision float into a vector register.
    VFrexpMantF16E32(crate::VectorUnary<family::VFrexpMant, crate::F16, crate::F16, crate::E32>),
    /// Extract the binary significand, or mantissa, of a half-precision float input and store the result as a half-precision float into a vector register.
    VFrexpMantF16E64(crate::VectorUnary<family::VFrexpMant, crate::F16, crate::F16, crate::E64>),
    /// Extract the binary significand, or mantissa, of a single-precision float input and store the result as a single-precision float into a vector register.
    VFrexpMantF32E32(crate::VectorUnary<family::VFrexpMant, crate::F32, crate::F32, crate::E32>),
    /// Extract the binary significand, or mantissa, of a single-precision float input and store the result as a single-precision float into a vector register.
    VFrexpMantF32E64(crate::VectorUnary<family::VFrexpMant, crate::F32, crate::F32, crate::E64>),
    /// Extract the binary significand, or mantissa, of a double-precision float input and store the result as a double-precision float into a vector register.
    VFrexpMantF64E32(crate::VectorUnary<family::VFrexpMant, crate::F64, crate::F64, crate::E32>),
    /// Extract the binary significand, or mantissa, of a double-precision float input and store the result as a double-precision float into a vector register.
    VFrexpMantF64E64(crate::VectorUnary<family::VFrexpMant, crate::F64, crate::F64, crate::E64>),
    /// Multiply the first input, a floating point value, by an integral power of 2 specified in the second input, a signed integer value, and store the floating point result into a vector register.
    VLdexpF16E32(
        crate::VectorBinary<family::VLdexp, crate::F16, crate::F16, crate::F16, crate::E32>,
    ),
    /// Multiply the first input, a floating point value, by an integral power of 2 specified in the second input, a signed integer value, and store the floating point result into a vector register.
    VLdexpF16E64(
        crate::VectorBinary<family::VLdexp, crate::F16, crate::F16, crate::F16, crate::E64>,
    ),
    /// Multiply the first input, a floating point value, by an integral power of 2 specified in the second input, a signed integer value, and store the floating point result into a vector register.
    VLdexpF32E64(
        crate::VectorBinary<family::VLdexp, crate::F32, crate::F32, crate::I32, crate::E64>,
    ),
    /// Multiply the first input, a floating point value, by an integral power of 2 specified in the second input, a signed integer value, and store the floating point result into a vector register.
    VLdexpF64E64(
        crate::VectorBinary<family::VLdexp, crate::F64, crate::F64, crate::I32, crate::E64>,
    ),
    /// Average two 4-D vectors stored as packed bytes in the first two inputs with rounding control provided by the third input, then store the result into a vector register. Each byte in the third input acts as a rounding mode for the corresponding element; if the LSB is set then 0.5 rounds up, otherwise 0.5 truncates.
    VLerpU8E64(
        crate::VectorTernary<
            family::VLerp,
            crate::U32,
            crate::B32,
            crate::B32,
            crate::B32,
            crate::E64,
        >,
    ),
    /// Calculate the base 2 logarithm of the half-precision float input and store the result into a vector register.
    VLogF16E32(crate::VectorUnary<family::VLog, crate::F16, crate::F16, crate::E32>),
    /// Calculate the base 2 logarithm of the half-precision float input and store the result into a vector register.
    VLogF16E64(crate::VectorUnary<family::VLog, crate::F16, crate::F16, crate::E64>),
    /// Calculate the base 2 logarithm of the single-precision float input and store the result into a vector register.
    VLogF32E32(crate::VectorUnary<family::VLog, crate::F32, crate::F32, crate::E32>),
    /// Calculate the base 2 logarithm of the single-precision float input and store the result into a vector register.
    VLogF32E64(crate::VectorUnary<family::VLog, crate::F32, crate::F32, crate::E64>),
    /// Given a shift count in the second input, calculate the logical shift left of the first input, then add the third input to the intermediate result, then store the final result into a vector register.
    VLshlAddU32E64(
        crate::VectorTernary<
            family::VLshlAdd,
            crate::U32,
            crate::U32,
            crate::U32,
            crate::U32,
            crate::E64,
        >,
    ),
    /// Given a shift count in the second input, calculate the logical shift left of the first input, then calculate the bitwise OR of the intermediate result and the third input, then store the final result into a vector register.
    VLshlOrB32E64(
        crate::VectorTernary<
            family::VLshlOr,
            crate::B32,
            crate::B32,
            crate::B32,
            crate::B32,
            crate::E64,
        >,
    ),
    /// Given a shift count in the first vector input, calculate the logical shift left of the second vector input and store the result into a vector register.
    VLshlrevB16E64(
        crate::VectorBinary<family::VLshlrev, crate::B16, crate::B16, crate::B16, crate::E64>,
    ),
    /// Given a shift count in the first vector input, calculate the logical shift left of the second vector input and store the result into a vector register.
    VLshlrevB32E32(
        crate::VectorBinary<family::VLshlrev, crate::B32, crate::B32, crate::B32, crate::E32>,
    ),
    /// Given a shift count in the first vector input, calculate the logical shift left of the second vector input and store the result into a vector register.
    VLshlrevB32E64(
        crate::VectorBinary<family::VLshlrev, crate::B32, crate::B32, crate::B32, crate::E64>,
    ),
    /// Given a shift count in the first vector input, calculate the logical shift left of the second vector input and store the result into a vector register.
    VLshlrevB64E64(
        crate::VectorBinary<family::VLshlrev, crate::B64, crate::U32, crate::B64, crate::E64>,
    ),
    /// Given a shift count in the first vector input, calculate the logical shift right of the second vector input and store the result into a vector register.
    VLshrrevB16E64(
        crate::VectorBinary<family::VLshrrev, crate::B16, crate::B16, crate::B16, crate::E64>,
    ),
    /// Given a shift count in the first vector input, calculate the logical shift right of the second vector input and store the result into a vector register.
    VLshrrevB32E32(
        crate::VectorBinary<family::VLshrrev, crate::B32, crate::B32, crate::B32, crate::E32>,
    ),
    /// Given a shift count in the first vector input, calculate the logical shift right of the second vector input and store the result into a vector register.
    VLshrrevB32E64(
        crate::VectorBinary<family::VLshrrev, crate::B32, crate::B32, crate::B32, crate::E64>,
    ),
    /// Given a shift count in the first vector input, calculate the logical shift right of the second vector input and store the result into a vector register.
    VLshrrevB64E64(
        crate::VectorBinary<family::VLshrrev, crate::B64, crate::U32, crate::B64, crate::E64>,
    ),
    /// Multiply two signed 16-bit integer inputs, add a signed 16-bit integer value from a third input, and store the result into a vector register.
    VMadI16E64(
        crate::VectorTernary<
            family::VMad,
            crate::I16,
            crate::I16,
            crate::I16,
            crate::I16,
            crate::E64,
        >,
    ),
    /// Multiply two signed 16-bit integer inputs in the signed 32-bit integer domain, add a signed 32-bit integer value from a third input, and store the result as a signed 32-bit integer into a vector register.
    VMadI32I16E64(
        crate::VectorTernary<
            family::VMad,
            crate::I32,
            crate::I16,
            crate::I16,
            crate::I32,
            crate::E64,
        >,
    ),
    /// Multiply two unsigned 16-bit integer inputs, add an unsigned 16-bit integer value from a third input, and store the result into a vector register.
    VMadU16E64(
        crate::VectorTernary<
            family::VMad,
            crate::U16,
            crate::U16,
            crate::U16,
            crate::U16,
            crate::E64,
        >,
    ),
    /// Multiply two unsigned 16-bit integer inputs in the unsigned 32-bit integer domain, add an unsigned 32-bit integer value from a third input, and store the result as an unsigned 32-bit integer into a vector register.
    VMadU32U16E64(
        crate::VectorTernary<
            family::VMad,
            crate::U32,
            crate::U16,
            crate::U16,
            crate::U32,
            crate::E64,
        >,
    ),
    /// Select the maximum of three half-precision float inputs and store the selected value into a vector register.
    VMax3F16E64(
        crate::VectorTernary<
            family::VMax3,
            crate::F16,
            crate::F16,
            crate::F16,
            crate::F16,
            crate::E64,
        >,
    ),
    /// Select the maximum of three single-precision float inputs and store the selected value into a vector register.
    VMax3F32E64(
        crate::VectorTernary<
            family::VMax3,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::E64,
        >,
    ),
    /// Select the maximum of three signed 16-bit integer inputs and store the selected value into a vector register.
    VMax3I16E64(
        crate::VectorTernary<
            family::VMax3,
            crate::I16,
            crate::I16,
            crate::I16,
            crate::I16,
            crate::E64,
        >,
    ),
    /// Select the maximum of three signed 32-bit integer inputs and store the selected value into a vector register.
    VMax3I32E64(
        crate::VectorTernary<
            family::VMax3,
            crate::I32,
            crate::I32,
            crate::I32,
            crate::I32,
            crate::E64,
        >,
    ),
    /// Select the maximum of three unsigned 16-bit integer inputs and store the selected value into a vector register.
    VMax3U16E64(
        crate::VectorTernary<
            family::VMax3,
            crate::U16,
            crate::U16,
            crate::U16,
            crate::U16,
            crate::E64,
        >,
    ),
    /// Select the maximum of three unsigned 32-bit integer inputs and store the selected value into a vector register.
    VMax3U32E64(
        crate::VectorTernary<
            family::VMax3,
            crate::U32,
            crate::U32,
            crate::U32,
            crate::U32,
            crate::E64,
        >,
    ),
    /// Select the maximum of two half-precision float inputs and store the selected value into a vector register.
    VMaxF16E32(crate::VectorBinary<family::VMax, crate::F16, crate::F16, crate::F16, crate::E32>),
    /// Select the maximum of two half-precision float inputs and store the selected value into a vector register.
    VMaxF16E64(crate::VectorBinary<family::VMax, crate::F16, crate::F16, crate::F16, crate::E64>),
    /// Select the maximum of two single-precision float inputs and store the selected value into a vector register.
    VMaxF32E32(crate::VectorBinary<family::VMax, crate::F32, crate::F32, crate::F32, crate::E32>),
    /// Select the maximum of two single-precision float inputs and store the selected value into a vector register.
    VMaxF32E64(crate::VectorBinary<family::VMax, crate::F32, crate::F32, crate::F32, crate::E64>),
    /// Select the maximum of two double-precision float inputs and store the selected value into a vector register.
    VMaxF64E64(crate::VectorBinary<family::VMax, crate::F64, crate::F64, crate::F64, crate::E64>),
    /// Select the maximum of two signed 16-bit integer inputs and store the selected value into a vector register.
    VMaxI16E64(crate::VectorBinary<family::VMax, crate::I16, crate::I16, crate::I16, crate::E64>),
    /// Select the maximum of two signed 32-bit integer inputs and store the selected value into a vector register.
    VMaxI32E32(crate::VectorBinary<family::VMax, crate::I32, crate::I32, crate::I32, crate::E32>),
    /// Select the maximum of two signed 32-bit integer inputs and store the selected value into a vector register.
    VMaxI32E64(crate::VectorBinary<family::VMax, crate::I32, crate::I32, crate::I32, crate::E64>),
    /// Select the maximum of two unsigned 16-bit integer inputs and store the selected value into a vector register.
    VMaxU16E64(crate::VectorBinary<family::VMax, crate::U16, crate::U16, crate::U16, crate::E64>),
    /// Select the maximum of two unsigned 32-bit integer inputs and store the selected value into a vector register.
    VMaxU32E32(crate::VectorBinary<family::VMax, crate::U32, crate::U32, crate::U32, crate::E32>),
    /// Select the maximum of two unsigned 32-bit integer inputs and store the selected value into a vector register.
    VMaxU32E64(crate::VectorBinary<family::VMax, crate::U32, crate::U32, crate::U32, crate::E64>),
    /// For each lane 32 <= N < 64, examine the N least significant bits of the first input and count how many of those bits are "1". For lane positions 0 <= N < 32 no bits are examined and the count is zero. Add this count to the value in the second input and store the result into a vector register.
    VMbcntHiU32B32E64(
        crate::VectorBinary<family::VMbcntHi, crate::B32, crate::B32, crate::B32, crate::E64>,
    ),
    /// For each lane 0 <= N < 32, examine the N least significant bits of the first input and count how many of those bits are "1". For each lane 32 <= N < 64, all "1" bits in the first input are counted. Add this count to the value in the second input and store the result into a vector register.
    VMbcntLoU32B32E64(
        crate::VectorBinary<family::VMbcntLo, crate::B32, crate::B32, crate::B32, crate::E64>,
    ),
    /// Select the median of three half-precision float inputs and store the selected value into a vector register.
    VMed3F16E64(
        crate::VectorTernary<
            family::VMed3,
            crate::F16,
            crate::F16,
            crate::F16,
            crate::F16,
            crate::E64,
        >,
    ),
    /// Select the median of three single-precision float inputs and store the selected value into a vector register.
    VMed3F32E64(
        crate::VectorTernary<
            family::VMed3,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::E64,
        >,
    ),
    /// Select the median of three signed 16-bit integer inputs and store the selected value into a vector register.
    VMed3I16E64(
        crate::VectorTernary<
            family::VMed3,
            crate::I16,
            crate::I16,
            crate::I16,
            crate::I16,
            crate::E64,
        >,
    ),
    /// Select the median of three signed 32-bit integer inputs and store the selected value into a vector register.
    VMed3I32E64(
        crate::VectorTernary<
            family::VMed3,
            crate::I32,
            crate::I32,
            crate::I32,
            crate::I32,
            crate::E64,
        >,
    ),
    /// Select the median of three unsigned 16-bit integer inputs and store the selected value into a vector register.
    VMed3U16E64(
        crate::VectorTernary<
            family::VMed3,
            crate::U16,
            crate::U16,
            crate::U16,
            crate::U16,
            crate::E64,
        >,
    ),
    /// Select the median of three unsigned 32-bit integer inputs and store the selected value into a vector register.
    VMed3U32E64(
        crate::VectorTernary<
            family::VMed3,
            crate::U32,
            crate::U32,
            crate::U32,
            crate::U32,
            crate::E64,
        >,
    ),
    /// Select the minimum of three half-precision float inputs and store the selected value into a vector register.
    VMin3F16E64(
        crate::VectorTernary<
            family::VMin3,
            crate::F16,
            crate::F16,
            crate::F16,
            crate::F16,
            crate::E64,
        >,
    ),
    /// Select the minimum of three single-precision float inputs and store the selected value into a vector register.
    VMin3F32E64(
        crate::VectorTernary<
            family::VMin3,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::E64,
        >,
    ),
    /// Select the minimum of three signed 16-bit integer inputs and store the selected value into a vector register.
    VMin3I16E64(
        crate::VectorTernary<
            family::VMin3,
            crate::I16,
            crate::I16,
            crate::I16,
            crate::I16,
            crate::E64,
        >,
    ),
    /// Select the minimum of three signed 32-bit integer inputs and store the selected value into a vector register.
    VMin3I32E64(
        crate::VectorTernary<
            family::VMin3,
            crate::I32,
            crate::I32,
            crate::I32,
            crate::I32,
            crate::E64,
        >,
    ),
    /// Select the minimum of three unsigned 16-bit integer inputs and store the selected value into a vector register.
    VMin3U16E64(
        crate::VectorTernary<
            family::VMin3,
            crate::U16,
            crate::U16,
            crate::U16,
            crate::U16,
            crate::E64,
        >,
    ),
    /// Select the minimum of three unsigned 32-bit integer inputs and store the selected value into a vector register.
    VMin3U32E64(
        crate::VectorTernary<
            family::VMin3,
            crate::U32,
            crate::U32,
            crate::U32,
            crate::U32,
            crate::E64,
        >,
    ),
    /// Select the minimum of two half-precision float inputs and store the selected value into a vector register.
    VMinF16E32(crate::VectorBinary<family::VMin, crate::F16, crate::F16, crate::F16, crate::E32>),
    /// Select the minimum of two half-precision float inputs and store the selected value into a vector register.
    VMinF16E64(crate::VectorBinary<family::VMin, crate::F16, crate::F16, crate::F16, crate::E64>),
    /// Select the minimum of two single-precision float inputs and store the selected value into a vector register.
    VMinF32E32(crate::VectorBinary<family::VMin, crate::F32, crate::F32, crate::F32, crate::E32>),
    /// Select the minimum of two single-precision float inputs and store the selected value into a vector register.
    VMinF32E64(crate::VectorBinary<family::VMin, crate::F32, crate::F32, crate::F32, crate::E64>),
    /// Select the minimum of two double-precision float inputs and store the selected value into a vector register.
    VMinF64E64(crate::VectorBinary<family::VMin, crate::F64, crate::F64, crate::F64, crate::E64>),
    /// Select the minimum of two signed 16-bit integer inputs and store the selected value into a vector register.
    VMinI16E64(crate::VectorBinary<family::VMin, crate::I16, crate::I16, crate::I16, crate::E64>),
    /// Select the minimum of two signed 32-bit integer inputs and store the selected value into a vector register.
    VMinI32E32(crate::VectorBinary<family::VMin, crate::I32, crate::I32, crate::I32, crate::E32>),
    /// Select the minimum of two signed 32-bit integer inputs and store the selected value into a vector register.
    VMinI32E64(crate::VectorBinary<family::VMin, crate::I32, crate::I32, crate::I32, crate::E64>),
    /// Select the minimum of two unsigned 16-bit integer inputs and store the selected value into a vector register.
    VMinU16E64(crate::VectorBinary<family::VMin, crate::U16, crate::U16, crate::U16, crate::E64>),
    /// Select the minimum of two unsigned 32-bit integer inputs and store the selected value into a vector register.
    VMinU32E32(crate::VectorBinary<family::VMin, crate::U32, crate::U32, crate::U32, crate::E32>),
    /// Select the minimum of two unsigned 32-bit integer inputs and store the selected value into a vector register.
    VMinU32E64(crate::VectorBinary<family::VMin, crate::U32, crate::U32, crate::U32, crate::E64>),
    /// Move 32-bit data from a vector input into a vector register.
    VMovB32E32(crate::VectorUnary<family::VMov, crate::B32, crate::B32, crate::E32>),
    /// Move 32-bit data from a vector input into a vector register.
    VMovB32E64(crate::VectorUnary<family::VMov, crate::B32, crate::B32, crate::E64>),
    /// Perform the V_MSAD_U8 operation four times using different slices of the first array, all entries of the second array and each entry of the third array. Truncate each result to 16 bits, pack the values into a 4-entry array and store the array into a vector register. The first input is an 8-entry array of unsigned 8-bit integers, the second input is a 4-entry array of unsigned 8-bit integers and the third input is a 4-entry array of unsigned 16-bit integers.
    VMqsadPkU16U8E64(
        crate::VectorTernary<
            family::VMqsadPk,
            crate::B64,
            crate::B64,
            crate::B32,
            crate::B64,
            crate::E64,
        >,
    ),
    /// Calculate the sum of absolute differences of elements in two packed 4-component unsigned 8-bit integer inputs, except that elements where the second input (known as the reference input) is zero are not included in the sum. Add an unsigned 32-bit integer value from the third input and store the result into a vector register.
    VMsadU8E64(
        crate::VectorTernary<
            family::VMsad,
            crate::U32,
            crate::B32,
            crate::B32,
            crate::B32,
            crate::E64,
        >,
    ),
    /// Multiply two floating point inputs and store the result into a vector register.
    VMulF16E32(crate::VectorBinary<family::VMul, crate::F16, crate::F16, crate::F16, crate::E32>),
    /// Multiply two floating point inputs and store the result into a vector register.
    VMulF16E64(crate::VectorBinary<family::VMul, crate::F16, crate::F16, crate::F16, crate::E64>),
    /// Multiply two floating point inputs and store the result into a vector register.
    VMulF32E32(crate::VectorBinary<family::VMul, crate::F32, crate::F32, crate::F32, crate::E32>),
    /// Multiply two floating point inputs and store the result into a vector register.
    VMulF32E64(crate::VectorBinary<family::VMul, crate::F32, crate::F32, crate::F32, crate::E64>),
    /// Multiply two floating point inputs and store the result into a vector register.
    VMulF64E64(crate::VectorBinary<family::VMul, crate::F64, crate::F64, crate::F64, crate::E64>),
    /// Multiply two signed 32-bit integer inputs and store the high 32 bits of the result into a vector register.
    VMulHiI32E64(
        crate::VectorBinary<family::VMulHi, crate::I32, crate::I32, crate::I32, crate::E64>,
    ),
    /// Multiply two unsigned 32-bit integer inputs and store the high 32 bits of the result into a vector register.
    VMulHiU32E64(
        crate::VectorBinary<family::VMulHi, crate::U32, crate::U32, crate::U32, crate::E64>,
    ),
    /// Multiply two floating point inputs and store the result into a vector register. Follows DX9 rules where 0.0 times anything produces 0.0 (this differs from other APIs when the other input is infinity or NaN).
    VMulLegacyF32E32(
        crate::VectorBinary<family::VMulLegacy, crate::F32, crate::F32, crate::F32, crate::E32>,
    ),
    /// Multiply two floating point inputs and store the result into a vector register. Follows DX9 rules where 0.0 times anything produces 0.0 (this differs from other APIs when the other input is infinity or NaN).
    VMulLegacyF32E64(
        crate::VectorBinary<family::VMulLegacy, crate::F32, crate::F32, crate::F32, crate::E64>,
    ),
    /// Multiply two unsigned 16-bit integer inputs and store the low bits of the result into a vector register.
    VMulLoU16E64(
        crate::VectorBinary<family::VMulLo, crate::U16, crate::U16, crate::U16, crate::E64>,
    ),
    /// Multiply two unsigned 32-bit integer inputs and store the result into a vector register.
    VMulLoU32E64(
        crate::VectorBinary<family::VMulLo, crate::U32, crate::U32, crate::U32, crate::E64>,
    ),
    /// Multiply two floating point inputs and store the result into a vector register. Specific rules apply to accommodate lighting calculations: 0.0 * x = 0.0 and alternate INF, NAN, overflow rules apply.
    VMullitF32E64(
        crate::VectorTernary<
            family::VMullit,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::F32,
            crate::E64,
        >,
    ),
    /// Calculate bitwise negation on a vector input and store the result into a vector register.
    VNotB32E32(crate::VectorUnary<family::VNot, crate::B32, crate::B32, crate::E32>),
    /// Calculate bitwise negation on a vector input and store the result into a vector register.
    VNotB32E64(crate::VectorUnary<family::VNot, crate::B32, crate::B32, crate::E64>),
    /// Calculate the bitwise OR of three vector inputs and store the result into a vector register.
    VOr3B32E64(
        crate::VectorTernary<
            family::VOr3,
            crate::B32,
            crate::B32,
            crate::B32,
            crate::B32,
            crate::E64,
        >,
    ),
    /// Calculate bitwise OR on two vector inputs and store the result into a vector register.
    VOrB32E32(crate::VectorBinary<family::VOr, crate::B32, crate::B32, crate::B32, crate::E32>),
    /// Calculate bitwise OR on two vector inputs and store the result into a vector register.
    VOrB32E64(crate::VectorBinary<family::VOr, crate::B32, crate::B32, crate::B32, crate::E64>),
    /// Pack two half-precision float values into a single 32-bit value and store the result into a vector register.
    VPackB32F16E64(
        crate::VectorBinary<family::VPack, crate::B32, crate::F16, crate::F16, crate::E64>,
    ),
    /// Perform the V_SAD_U8 operation four times using different slices of the first array, all entries of the second array and each entry of the third array. Truncate each result to 16 bits, pack the values into a 4-entry array and store the array into a vector register. The first input is an 8-entry array of unsigned 8-bit integers, the second input is a 4-entry array of unsigned 8-bit integers and the third input is a 4-entry array of unsigned 16-bit integers.
    VQsadPkU16U8E64(
        crate::VectorTernary<
            family::VQsadPk,
            crate::B64,
            crate::B64,
            crate::B32,
            crate::B64,
            crate::E64,
        >,
    ),
    /// Calculate the reciprocal of the half-precision float input using IEEE rules and store the result into a vector register.
    VRcpF16E32(crate::VectorUnary<family::VRcp, crate::F16, crate::F16, crate::E32>),
    /// Calculate the reciprocal of the half-precision float input using IEEE rules and store the result into a vector register.
    VRcpF16E64(crate::VectorUnary<family::VRcp, crate::F16, crate::F16, crate::E64>),
    /// Calculate the reciprocal of the single-precision float input using IEEE rules and store the result into a vector register.
    VRcpF32E32(crate::VectorUnary<family::VRcp, crate::F32, crate::F32, crate::E32>),
    /// Calculate the reciprocal of the single-precision float input using IEEE rules and store the result into a vector register.
    VRcpF32E64(crate::VectorUnary<family::VRcp, crate::F32, crate::F32, crate::E64>),
    /// Calculate the reciprocal of the double-precision float input using IEEE rules and store the result into a vector register.
    VRcpF64E32(crate::VectorUnary<family::VRcp, crate::F64, crate::F64, crate::E32>),
    /// Calculate the reciprocal of the double-precision float input using IEEE rules and store the result into a vector register.
    VRcpF64E64(crate::VectorUnary<family::VRcp, crate::F64, crate::F64, crate::E64>),
    /// Calculate the reciprocal of the vector float input in a manner suitable for integer division and store the result into a vector register. This opcode is intended for use as part of an integer division macro.
    VRcpIflagF32E32(crate::VectorUnary<family::VRcpIflag, crate::F32, crate::F32, crate::E32>),
    /// Calculate the reciprocal of the vector float input in a manner suitable for integer division and store the result into a vector register. This opcode is intended for use as part of an integer division macro.
    VRcpIflagF32E64(crate::VectorUnary<family::VRcpIflag, crate::F32, crate::F32, crate::E64>),
    /// Round the half-precision float input to the nearest even integer and store the result in floating point format into a vector register.
    VRndneF16E32(crate::VectorUnary<family::VRndne, crate::F16, crate::F16, crate::E32>),
    /// Round the half-precision float input to the nearest even integer and store the result in floating point format into a vector register.
    VRndneF16E64(crate::VectorUnary<family::VRndne, crate::F16, crate::F16, crate::E64>),
    /// Round the single-precision float input to the nearest even integer and store the result in floating point format into a vector register.
    VRndneF32E32(crate::VectorUnary<family::VRndne, crate::F32, crate::F32, crate::E32>),
    /// Round the single-precision float input to the nearest even integer and store the result in floating point format into a vector register.
    VRndneF32E64(crate::VectorUnary<family::VRndne, crate::F32, crate::F32, crate::E64>),
    /// Round the double-precision float input to the nearest even integer and store the result in floating point format into a vector register.
    VRndneF64E32(crate::VectorUnary<family::VRndne, crate::F64, crate::F64, crate::E32>),
    /// Round the double-precision float input to the nearest even integer and store the result in floating point format into a vector register.
    VRndneF64E64(crate::VectorUnary<family::VRndne, crate::F64, crate::F64, crate::E64>),
    /// Calculate the reciprocal of the square root of the half-precision float input using IEEE rules and store the result into a vector register.
    VRsqF16E32(crate::VectorUnary<family::VRsq, crate::F16, crate::F16, crate::E32>),
    /// Calculate the reciprocal of the square root of the half-precision float input using IEEE rules and store the result into a vector register.
    VRsqF16E64(crate::VectorUnary<family::VRsq, crate::F16, crate::F16, crate::E64>),
    /// Calculate the reciprocal of the square root of the single-precision float input using IEEE rules and store the result into a vector register.
    VRsqF32E32(crate::VectorUnary<family::VRsq, crate::F32, crate::F32, crate::E32>),
    /// Calculate the reciprocal of the square root of the single-precision float input using IEEE rules and store the result into a vector register.
    VRsqF32E64(crate::VectorUnary<family::VRsq, crate::F32, crate::F32, crate::E64>),
    /// Calculate the reciprocal of the square root of the double-precision float input using IEEE rules and store the result into a vector register.
    VRsqF64E32(crate::VectorUnary<family::VRsq, crate::F64, crate::F64, crate::E32>),
    /// Calculate the reciprocal of the square root of the double-precision float input using IEEE rules and store the result into a vector register.
    VRsqF64E64(crate::VectorUnary<family::VRsq, crate::F64, crate::F64, crate::E64>),
    /// Calculate the absolute difference of two unsigned 32-bit integer inputs, add an unsigned 32-bit integer value from the third input and store the result into a vector register.
    VSadU32E64(
        crate::VectorTernary<
            family::VSad,
            crate::U32,
            crate::U32,
            crate::U32,
            crate::U32,
            crate::E64,
        >,
    ),
    /// Calculate the trigonometric sine of a half-precision float value using IEEE rules and store the result into a vector register. The operand is calculated by scaling the vector input by 2 PI.
    VSinF16E32(crate::VectorUnary<family::VSin, crate::F16, crate::F16, crate::E32>),
    /// Calculate the trigonometric sine of a half-precision float value using IEEE rules and store the result into a vector register. The operand is calculated by scaling the vector input by 2 PI.
    VSinF16E64(crate::VectorUnary<family::VSin, crate::F16, crate::F16, crate::E64>),
    /// Calculate the trigonometric sine of a single-precision float value using IEEE rules and store the result into a vector register. The operand is calculated by scaling the vector input by 2 PI.
    VSinF32E32(crate::VectorUnary<family::VSin, crate::F32, crate::F32, crate::E32>),
    /// Calculate the trigonometric sine of a single-precision float value using IEEE rules and store the result into a vector register. The operand is calculated by scaling the vector input by 2 PI.
    VSinF32E64(crate::VectorUnary<family::VSin, crate::F32, crate::F32, crate::E64>),
    /// Calculate the square root of the half-precision float input using IEEE rules and store the result into a vector register.
    VSqrtF16E32(crate::VectorUnary<family::VSqrt, crate::F16, crate::F16, crate::E32>),
    /// Calculate the square root of the half-precision float input using IEEE rules and store the result into a vector register.
    VSqrtF16E64(crate::VectorUnary<family::VSqrt, crate::F16, crate::F16, crate::E64>),
    /// Calculate the square root of the single-precision float input using IEEE rules and store the result into a vector register.
    VSqrtF32E32(crate::VectorUnary<family::VSqrt, crate::F32, crate::F32, crate::E32>),
    /// Calculate the square root of the single-precision float input using IEEE rules and store the result into a vector register.
    VSqrtF32E64(crate::VectorUnary<family::VSqrt, crate::F32, crate::F32, crate::E64>),
    /// Calculate the square root of the double-precision float input using IEEE rules and store the result into a vector register.
    VSqrtF64E32(crate::VectorUnary<family::VSqrt, crate::F64, crate::F64, crate::E32>),
    /// Calculate the square root of the double-precision float input using IEEE rules and store the result into a vector register.
    VSqrtF64E64(crate::VectorUnary<family::VSqrt, crate::F64, crate::F64, crate::E64>),
    /// Subtract the second floating point input from the first input and store the result into a vector register.
    VSubF16E32(crate::VectorBinary<family::VSub, crate::F16, crate::F16, crate::F16, crate::E32>),
    /// Subtract the second floating point input from the first input and store the result into a vector register.
    VSubF16E64(crate::VectorBinary<family::VSub, crate::F16, crate::F16, crate::F16, crate::E64>),
    /// Subtract the second floating point input from the first input and store the result into a vector register.
    VSubF32E32(crate::VectorBinary<family::VSub, crate::F32, crate::F32, crate::F32, crate::E32>),
    /// Subtract the second floating point input from the first input and store the result into a vector register.
    VSubF32E64(crate::VectorBinary<family::VSub, crate::F32, crate::F32, crate::F32, crate::E64>),
    /// Subtract the second signed 16-bit integer input from the first input and store the result into a vector register. No carry-in or carry-out support.
    VSubNcI16E64(
        crate::VectorBinary<family::VSubNc, crate::I16, crate::I16, crate::I16, crate::E64>,
    ),
    /// Subtract the second signed 32-bit integer input from the first input and store the result into a vector register. No carry-in or carry-out support.
    VSubNcI32E64(
        crate::VectorBinary<family::VSubNc, crate::I32, crate::I32, crate::I32, crate::E64>,
    ),
    /// Subtract the second unsigned 16-bit integer input from the first input and store the result into a vector register. No carry-in or carry-out support.
    VSubNcU16E64(
        crate::VectorBinary<family::VSubNc, crate::U16, crate::U16, crate::U16, crate::E64>,
    ),
    /// Subtract the second unsigned 32-bit integer input from the first input and store the result into a vector register. No carry-in or carry-out support.
    VSubNcU32E32(
        crate::VectorBinary<family::VSubNc, crate::U32, crate::U32, crate::U32, crate::E32>,
    ),
    /// Subtract the second unsigned 32-bit integer input from the first input and store the result into a vector register. No carry-in or carry-out support.
    VSubNcU32E64(
        crate::VectorBinary<family::VSubNc, crate::U32, crate::U32, crate::U32, crate::E64>,
    ),
    /// Subtract the first floating point input from the second input and store the result into a vector register.
    VSubrevF16E32(
        crate::VectorBinary<family::VSubrev, crate::F16, crate::F16, crate::F16, crate::E32>,
    ),
    /// Subtract the first floating point input from the second input and store the result into a vector register.
    VSubrevF16E64(
        crate::VectorBinary<family::VSubrev, crate::F16, crate::F16, crate::F16, crate::E64>,
    ),
    /// Subtract the first floating point input from the second input and store the result into a vector register.
    VSubrevF32E32(
        crate::VectorBinary<family::VSubrev, crate::F32, crate::F32, crate::F32, crate::E32>,
    ),
    /// Subtract the first floating point input from the second input and store the result into a vector register.
    VSubrevF32E64(
        crate::VectorBinary<family::VSubrev, crate::F32, crate::F32, crate::F32, crate::E64>,
    ),
    /// Subtract the first unsigned 32-bit integer input from the second input and store the result into a vector register. No carry-in or carry-out support.
    VSubrevNcU32E32(
        crate::VectorBinary<family::VSubrevNc, crate::U32, crate::U32, crate::U32, crate::E32>,
    ),
    /// Subtract the first unsigned 32-bit integer input from the second input and store the result into a vector register. No carry-in or carry-out support.
    VSubrevNcU32E64(
        crate::VectorBinary<family::VSubrevNc, crate::U32, crate::U32, crate::U32, crate::E64>,
    ),
    /// Look up a 53-bit segment of 2/PI using an integer segment select in the second input. Scale the intermediate result by the exponent from the first double-precision float input and store the double-precision float result into a vector register.
    VTrigPreopF64E64(
        crate::VectorBinary<family::VTrigPreop, crate::F64, crate::F64, crate::B32, crate::E64>,
    ),
    /// Compute the integer part of a half-precision float input using round toward zero semantics and store the result in floating point format into a vector register.
    VTruncF16E32(crate::VectorUnary<family::VTrunc, crate::F16, crate::F16, crate::E32>),
    /// Compute the integer part of a half-precision float input using round toward zero semantics and store the result in floating point format into a vector register.
    VTruncF16E64(crate::VectorUnary<family::VTrunc, crate::F16, crate::F16, crate::E64>),
    /// Compute the integer part of a single-precision float input using round toward zero semantics and store the result in floating point format into a vector register.
    VTruncF32E32(crate::VectorUnary<family::VTrunc, crate::F32, crate::F32, crate::E32>),
    /// Compute the integer part of a single-precision float input using round toward zero semantics and store the result in floating point format into a vector register.
    VTruncF32E64(crate::VectorUnary<family::VTrunc, crate::F32, crate::F32, crate::E64>),
    /// Compute the integer part of a double-precision float input using round toward zero semantics and store the result in floating point format into a vector register.
    VTruncF64E32(crate::VectorUnary<family::VTrunc, crate::F64, crate::F64, crate::E32>),
    /// Compute the integer part of a double-precision float input using round toward zero semantics and store the result in floating point format into a vector register.
    VTruncF64E64(crate::VectorUnary<family::VTrunc, crate::F64, crate::F64, crate::E64>),
    /// Calculate bitwise XOR of the first two vector inputs, then add the third vector input to the intermediate result, then store the final result into a vector register.
    VXadU32E64(
        crate::VectorTernary<
            family::VXad,
            crate::U32,
            crate::U32,
            crate::U32,
            crate::U32,
            crate::E64,
        >,
    ),
    /// Calculate bitwise XNOR on two vector inputs and store the result into a vector register.
    VXnorB32E32(crate::VectorBinary<family::VXnor, crate::B32, crate::B32, crate::B32, crate::E32>),
    /// Calculate bitwise XNOR on two vector inputs and store the result into a vector register.
    VXnorB32E64(crate::VectorBinary<family::VXnor, crate::B32, crate::B32, crate::B32, crate::E64>),
    /// Calculate the bitwise XOR of three vector inputs and store the result into a vector register.
    VXor3B32E64(
        crate::VectorTernary<
            family::VXor3,
            crate::B32,
            crate::B32,
            crate::B32,
            crate::B32,
            crate::E64,
        >,
    ),
    /// Calculate bitwise XOR on two vector inputs and store the result into a vector register.
    VXorB32E32(crate::VectorBinary<family::VXor, crate::B32, crate::B32, crate::B32, crate::E32>),
    /// Calculate bitwise XOR on two vector inputs and store the result into a vector register.
    VXorB32E64(crate::VectorBinary<family::VXor, crate::B32, crate::B32, crate::B32, crate::E64>),
}
pub fn parse(text: &str) -> Result<DecodedInstruction, crate::DecodeError> {
    match text.split_whitespace().next().unwrap_or("") {
        "v_add3_u32_e64" => Ok(DecodedInstruction::VAdd3U32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VAdd3,
                crate::U32,
                crate::U32,
                crate::U32,
                crate::U32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_add_f16_e32" | "v_add_f16" => {
            Ok(DecodedInstruction::VAddF16E32({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VAdd , crate :: F16 , crate :: F16 , crate :: F16 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_add_f16_e64" => {
            Ok(DecodedInstruction::VAddF16E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VAdd , crate :: F16 , crate :: F16 , crate :: F16 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_add_f32_e32" | "v_add_f32" => {
            Ok(DecodedInstruction::VAddF32E32({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VAdd , crate :: F32 , crate :: F32 , crate :: F32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_add_f32_e64" => {
            Ok(DecodedInstruction::VAddF32E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VAdd , crate :: F32 , crate :: F32 , crate :: F32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_add_f64_e64" => {
            Ok(DecodedInstruction::VAddF64E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VAdd , crate :: F64 , crate :: F64 , crate :: F64 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_add_lshl_u32_e64" => Ok(DecodedInstruction::VAddLshlU32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VAddLshl,
                crate::U32,
                crate::U32,
                crate::U32,
                crate::U32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_add_nc_i16_e64" => Ok(DecodedInstruction::VAddNcI16E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VAddNc , crate :: I16 , crate :: I16 , crate :: I16 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_add_nc_i32_e64" => Ok(DecodedInstruction::VAddNcI32E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VAddNc , crate :: I32 , crate :: I32 , crate :: I32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_add_nc_u16_e64" => Ok(DecodedInstruction::VAddNcU16E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VAddNc , crate :: U16 , crate :: U16 , crate :: U16 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_add_nc_u32_e32" | "v_add_nc_u32" => Ok(DecodedInstruction::VAddNcU32E32({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VAddNc , crate :: U32 , crate :: U32 , crate :: U32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_add_nc_u32_e64" => Ok(DecodedInstruction::VAddNcU32E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VAddNc , crate :: U32 , crate :: U32 , crate :: U32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_alignbit_b32_e64" => Ok(DecodedInstruction::VAlignbitB32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VAlignbit,
                crate::B32,
                crate::B32,
                crate::B32,
                crate::U8,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_alignbyte_b32_e64" => Ok(DecodedInstruction::VAlignbyteB32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VAlignbyte,
                crate::B32,
                crate::B32,
                crate::B32,
                crate::U8,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_and_b32_e32" | "v_and_b32" => {
            Ok(DecodedInstruction::VAndB32E32({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VAnd , crate :: B32 , crate :: B32 , crate :: B32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_and_b32_e64" => {
            Ok(DecodedInstruction::VAndB32E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VAnd , crate :: B32 , crate :: B32 , crate :: B32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_and_or_b32_e64" => Ok(DecodedInstruction::VAndOrB32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VAndOr,
                crate::B32,
                crate::B32,
                crate::B32,
                crate::B32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_ashrrev_i16_e64" => Ok(DecodedInstruction::VAshrrevI16E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VAshrrev , crate :: I16 , crate :: U16 , crate :: I16 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_ashrrev_i32_e32" | "v_ashrrev_i32" => Ok(DecodedInstruction::VAshrrevI32E32({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VAshrrev , crate :: I32 , crate :: U32 , crate :: I32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_ashrrev_i32_e64" => Ok(DecodedInstruction::VAshrrevI32E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VAshrrev , crate :: I32 , crate :: U32 , crate :: I32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_ashrrev_i64_e64" => Ok(DecodedInstruction::VAshrrevI64E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VAshrrev , crate :: I64 , crate :: U32 , crate :: I64 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_bcnt_u32_b32_e64" => Ok(DecodedInstruction::VBcntU32B32E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VBcnt , crate :: B32 , crate :: B32 , crate :: B32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_bfe_i32_e64" => Ok(DecodedInstruction::VBfeI32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VBfe,
                crate::I32,
                crate::I32,
                crate::U32,
                crate::U32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_bfe_u32_e64" => Ok(DecodedInstruction::VBfeU32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VBfe,
                crate::U32,
                crate::U32,
                crate::U32,
                crate::U32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_bfi_b32_e64" => Ok(DecodedInstruction::VBfiB32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VBfi,
                crate::B32,
                crate::B32,
                crate::B32,
                crate::B32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_bfm_b32_e64" => {
            Ok(DecodedInstruction::VBfmB32E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VBfm , crate :: B32 , crate :: B32 , crate :: B32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_bfrev_b32_e32" | "v_bfrev_b32" => Ok(DecodedInstruction::VBfrevB32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VBfrev, crate::B32, crate::B32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_bfrev_b32_e64" => Ok(DecodedInstruction::VBfrevB32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VBfrev, crate::B32, crate::B32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_ceil_f16_e32" | "v_ceil_f16" => Ok(DecodedInstruction::VCeilF16E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCeil, crate::F16, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_ceil_f16_e64" => Ok(DecodedInstruction::VCeilF16E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCeil, crate::F16, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_ceil_f32_e32" | "v_ceil_f32" => Ok(DecodedInstruction::VCeilF32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCeil, crate::F32, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_ceil_f32_e64" => Ok(DecodedInstruction::VCeilF32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCeil, crate::F32, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_ceil_f64_e32" | "v_ceil_f64" => Ok(DecodedInstruction::VCeilF64E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCeil, crate::F64, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_ceil_f64_e64" => Ok(DecodedInstruction::VCeilF64E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCeil, crate::F64, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cmp_eq_f16_e32" | "v_cmp_eq_f16" => Ok(DecodedInstruction::VCmpEqF16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Eq, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_eq_f16_e64" => Ok(DecodedInstruction::VCmpEqF16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Eq, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_eq_f32_e32" | "v_cmp_eq_f32" => Ok(DecodedInstruction::VCmpEqF32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Eq, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_eq_f32_e64" => Ok(DecodedInstruction::VCmpEqF32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Eq, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_eq_f64_e32" | "v_cmp_eq_f64" => Ok(DecodedInstruction::VCmpEqF64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Eq, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_eq_f64_e64" => Ok(DecodedInstruction::VCmpEqF64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Eq, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_eq_i16_e32" | "v_cmp_eq_i16" => Ok(DecodedInstruction::VCmpEqI16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Eq, crate::I16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_eq_i16_e64" => Ok(DecodedInstruction::VCmpEqI16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Eq, crate::I16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_eq_i32_e32" | "v_cmp_eq_i32" => Ok(DecodedInstruction::VCmpEqI32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Eq, crate::I32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_eq_i32_e64" => Ok(DecodedInstruction::VCmpEqI32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Eq, crate::I32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_eq_i64_e32" | "v_cmp_eq_i64" => Ok(DecodedInstruction::VCmpEqI64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Eq, crate::I64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_eq_i64_e64" => Ok(DecodedInstruction::VCmpEqI64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Eq, crate::I64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_eq_u16_e32" | "v_cmp_eq_u16" => Ok(DecodedInstruction::VCmpEqU16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Eq, crate::U16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_eq_u16_e64" => Ok(DecodedInstruction::VCmpEqU16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Eq, crate::U16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_eq_u32_e32" | "v_cmp_eq_u32" => Ok(DecodedInstruction::VCmpEqU32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Eq, crate::U32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_eq_u32_e64" => Ok(DecodedInstruction::VCmpEqU32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Eq, crate::U32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_eq_u64_e32" | "v_cmp_eq_u64" => Ok(DecodedInstruction::VCmpEqU64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Eq, crate::U64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_eq_u64_e64" => Ok(DecodedInstruction::VCmpEqU64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Eq, crate::U64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_f_f16_e32" | "v_cmp_f_f16" => Ok(DecodedInstruction::VCmpFF16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Never, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_f_f16_e64" => Ok(DecodedInstruction::VCmpFF16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Never, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_f_f32_e32" | "v_cmp_f_f32" => Ok(DecodedInstruction::VCmpFF32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Never, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_f_f32_e64" => Ok(DecodedInstruction::VCmpFF32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Never, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_f_f64_e32" | "v_cmp_f_f64" => Ok(DecodedInstruction::VCmpFF64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Never, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_f_f64_e64" => Ok(DecodedInstruction::VCmpFF64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Never, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_f_i32_e32" | "v_cmp_f_i32" => Ok(DecodedInstruction::VCmpFI32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Never, crate::I32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_f_i32_e64" => Ok(DecodedInstruction::VCmpFI32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Never, crate::I32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_f_i64_e32" | "v_cmp_f_i64" => Ok(DecodedInstruction::VCmpFI64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Never, crate::I64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_f_i64_e64" => Ok(DecodedInstruction::VCmpFI64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Never, crate::I64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_f_u32_e32" | "v_cmp_f_u32" => Ok(DecodedInstruction::VCmpFU32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Never, crate::U32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_f_u32_e64" => Ok(DecodedInstruction::VCmpFU32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Never, crate::U32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_f_u64_e32" | "v_cmp_f_u64" => Ok(DecodedInstruction::VCmpFU64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Never, crate::U64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_f_u64_e64" => Ok(DecodedInstruction::VCmpFU64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Never, crate::U64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ge_f16_e32" | "v_cmp_ge_f16" => Ok(DecodedInstruction::VCmpGeF16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ge, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ge_f16_e64" => Ok(DecodedInstruction::VCmpGeF16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ge, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ge_f32_e32" | "v_cmp_ge_f32" => Ok(DecodedInstruction::VCmpGeF32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ge, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ge_f32_e64" => Ok(DecodedInstruction::VCmpGeF32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ge, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ge_f64_e32" | "v_cmp_ge_f64" => Ok(DecodedInstruction::VCmpGeF64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ge, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ge_f64_e64" => Ok(DecodedInstruction::VCmpGeF64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ge, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ge_i16_e32" | "v_cmp_ge_i16" => Ok(DecodedInstruction::VCmpGeI16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ge, crate::I16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ge_i16_e64" => Ok(DecodedInstruction::VCmpGeI16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ge, crate::I16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ge_i32_e32" | "v_cmp_ge_i32" => Ok(DecodedInstruction::VCmpGeI32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ge, crate::I32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ge_i32_e64" => Ok(DecodedInstruction::VCmpGeI32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ge, crate::I32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ge_i64_e32" | "v_cmp_ge_i64" => Ok(DecodedInstruction::VCmpGeI64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ge, crate::I64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ge_i64_e64" => Ok(DecodedInstruction::VCmpGeI64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ge, crate::I64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ge_u16_e32" | "v_cmp_ge_u16" => Ok(DecodedInstruction::VCmpGeU16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ge, crate::U16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ge_u16_e64" => Ok(DecodedInstruction::VCmpGeU16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ge, crate::U16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ge_u32_e32" | "v_cmp_ge_u32" => Ok(DecodedInstruction::VCmpGeU32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ge, crate::U32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ge_u32_e64" => Ok(DecodedInstruction::VCmpGeU32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ge, crate::U32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ge_u64_e32" | "v_cmp_ge_u64" => Ok(DecodedInstruction::VCmpGeU64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ge, crate::U64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ge_u64_e64" => Ok(DecodedInstruction::VCmpGeU64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ge, crate::U64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_gt_f16_e32" | "v_cmp_gt_f16" => Ok(DecodedInstruction::VCmpGtF16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Gt, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_gt_f16_e64" => Ok(DecodedInstruction::VCmpGtF16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Gt, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_gt_f32_e32" | "v_cmp_gt_f32" => Ok(DecodedInstruction::VCmpGtF32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Gt, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_gt_f32_e64" => Ok(DecodedInstruction::VCmpGtF32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Gt, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_gt_f64_e32" | "v_cmp_gt_f64" => Ok(DecodedInstruction::VCmpGtF64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Gt, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_gt_f64_e64" => Ok(DecodedInstruction::VCmpGtF64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Gt, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_gt_i16_e32" | "v_cmp_gt_i16" => Ok(DecodedInstruction::VCmpGtI16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Gt, crate::I16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_gt_i16_e64" => Ok(DecodedInstruction::VCmpGtI16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Gt, crate::I16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_gt_i32_e32" | "v_cmp_gt_i32" => Ok(DecodedInstruction::VCmpGtI32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Gt, crate::I32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_gt_i32_e64" => Ok(DecodedInstruction::VCmpGtI32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Gt, crate::I32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_gt_i64_e32" | "v_cmp_gt_i64" => Ok(DecodedInstruction::VCmpGtI64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Gt, crate::I64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_gt_i64_e64" => Ok(DecodedInstruction::VCmpGtI64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Gt, crate::I64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_gt_u16_e32" | "v_cmp_gt_u16" => Ok(DecodedInstruction::VCmpGtU16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Gt, crate::U16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_gt_u16_e64" => Ok(DecodedInstruction::VCmpGtU16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Gt, crate::U16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_gt_u32_e32" | "v_cmp_gt_u32" => Ok(DecodedInstruction::VCmpGtU32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Gt, crate::U32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_gt_u32_e64" => Ok(DecodedInstruction::VCmpGtU32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Gt, crate::U32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_gt_u64_e32" | "v_cmp_gt_u64" => Ok(DecodedInstruction::VCmpGtU64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Gt, crate::U64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_gt_u64_e64" => Ok(DecodedInstruction::VCmpGtU64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Gt, crate::U64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_le_f16_e32" | "v_cmp_le_f16" => Ok(DecodedInstruction::VCmpLeF16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Le, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_le_f16_e64" => Ok(DecodedInstruction::VCmpLeF16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Le, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_le_f32_e32" | "v_cmp_le_f32" => Ok(DecodedInstruction::VCmpLeF32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Le, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_le_f32_e64" => Ok(DecodedInstruction::VCmpLeF32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Le, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_le_f64_e32" | "v_cmp_le_f64" => Ok(DecodedInstruction::VCmpLeF64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Le, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_le_f64_e64" => Ok(DecodedInstruction::VCmpLeF64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Le, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_le_i16_e32" | "v_cmp_le_i16" => Ok(DecodedInstruction::VCmpLeI16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Le, crate::I16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_le_i16_e64" => Ok(DecodedInstruction::VCmpLeI16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Le, crate::I16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_le_i32_e32" | "v_cmp_le_i32" => Ok(DecodedInstruction::VCmpLeI32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Le, crate::I32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_le_i32_e64" => Ok(DecodedInstruction::VCmpLeI32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Le, crate::I32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_le_i64_e32" | "v_cmp_le_i64" => Ok(DecodedInstruction::VCmpLeI64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Le, crate::I64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_le_i64_e64" => Ok(DecodedInstruction::VCmpLeI64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Le, crate::I64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_le_u16_e32" | "v_cmp_le_u16" => Ok(DecodedInstruction::VCmpLeU16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Le, crate::U16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_le_u16_e64" => Ok(DecodedInstruction::VCmpLeU16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Le, crate::U16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_le_u32_e32" | "v_cmp_le_u32" => Ok(DecodedInstruction::VCmpLeU32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Le, crate::U32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_le_u32_e64" => Ok(DecodedInstruction::VCmpLeU32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Le, crate::U32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_le_u64_e32" | "v_cmp_le_u64" => Ok(DecodedInstruction::VCmpLeU64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Le, crate::U64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_le_u64_e64" => Ok(DecodedInstruction::VCmpLeU64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Le, crate::U64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lg_f16_e32" | "v_cmp_lg_f16" => Ok(DecodedInstruction::VCmpLgF16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lg, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lg_f16_e64" => Ok(DecodedInstruction::VCmpLgF16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lg, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lg_f32_e32" | "v_cmp_lg_f32" => Ok(DecodedInstruction::VCmpLgF32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lg, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lg_f32_e64" => Ok(DecodedInstruction::VCmpLgF32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lg, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lg_f64_e32" | "v_cmp_lg_f64" => Ok(DecodedInstruction::VCmpLgF64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lg, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lg_f64_e64" => Ok(DecodedInstruction::VCmpLgF64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lg, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lt_f16_e32" | "v_cmp_lt_f16" => Ok(DecodedInstruction::VCmpLtF16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lt, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lt_f16_e64" => Ok(DecodedInstruction::VCmpLtF16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lt, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lt_f32_e32" | "v_cmp_lt_f32" => Ok(DecodedInstruction::VCmpLtF32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lt, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lt_f32_e64" => Ok(DecodedInstruction::VCmpLtF32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lt, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lt_f64_e32" | "v_cmp_lt_f64" => Ok(DecodedInstruction::VCmpLtF64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lt, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lt_f64_e64" => Ok(DecodedInstruction::VCmpLtF64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lt, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lt_i16_e32" | "v_cmp_lt_i16" => Ok(DecodedInstruction::VCmpLtI16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lt, crate::I16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lt_i16_e64" => Ok(DecodedInstruction::VCmpLtI16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lt, crate::I16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lt_i32_e32" | "v_cmp_lt_i32" => Ok(DecodedInstruction::VCmpLtI32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lt, crate::I32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lt_i32_e64" => Ok(DecodedInstruction::VCmpLtI32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lt, crate::I32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lt_i64_e32" | "v_cmp_lt_i64" => Ok(DecodedInstruction::VCmpLtI64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lt, crate::I64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lt_i64_e64" => Ok(DecodedInstruction::VCmpLtI64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lt, crate::I64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lt_u16_e32" | "v_cmp_lt_u16" => Ok(DecodedInstruction::VCmpLtU16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lt, crate::U16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lt_u16_e64" => Ok(DecodedInstruction::VCmpLtU16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lt, crate::U16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lt_u32_e32" | "v_cmp_lt_u32" => Ok(DecodedInstruction::VCmpLtU32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lt, crate::U32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lt_u32_e64" => Ok(DecodedInstruction::VCmpLtU32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lt, crate::U32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lt_u64_e32" | "v_cmp_lt_u64" => Ok(DecodedInstruction::VCmpLtU64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lt, crate::U64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_lt_u64_e64" => Ok(DecodedInstruction::VCmpLtU64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Lt, crate::U64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ne_i16_e32" | "v_cmp_ne_i16" => Ok(DecodedInstruction::VCmpNeI16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ne, crate::I16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ne_i16_e64" => Ok(DecodedInstruction::VCmpNeI16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ne, crate::I16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ne_i32_e32" | "v_cmp_ne_i32" => Ok(DecodedInstruction::VCmpNeI32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ne, crate::I32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ne_i32_e64" => Ok(DecodedInstruction::VCmpNeI32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ne, crate::I32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ne_i64_e32" | "v_cmp_ne_i64" => Ok(DecodedInstruction::VCmpNeI64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ne, crate::I64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ne_i64_e64" => Ok(DecodedInstruction::VCmpNeI64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ne, crate::I64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ne_u16_e32" | "v_cmp_ne_u16" => Ok(DecodedInstruction::VCmpNeU16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ne, crate::U16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ne_u16_e64" => Ok(DecodedInstruction::VCmpNeU16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ne, crate::U16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ne_u32_e32" | "v_cmp_ne_u32" => Ok(DecodedInstruction::VCmpNeU32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ne, crate::U32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ne_u32_e64" => Ok(DecodedInstruction::VCmpNeU32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ne, crate::U32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ne_u64_e32" | "v_cmp_ne_u64" => Ok(DecodedInstruction::VCmpNeU64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ne, crate::U64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ne_u64_e64" => Ok(DecodedInstruction::VCmpNeU64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ne, crate::U64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nge_f16_e32" | "v_cmp_nge_f16" => Ok(DecodedInstruction::VCmpNgeF16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotGe, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nge_f16_e64" => Ok(DecodedInstruction::VCmpNgeF16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotGe, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nge_f32_e32" | "v_cmp_nge_f32" => Ok(DecodedInstruction::VCmpNgeF32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotGe, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nge_f32_e64" => Ok(DecodedInstruction::VCmpNgeF32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotGe, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nge_f64_e32" | "v_cmp_nge_f64" => Ok(DecodedInstruction::VCmpNgeF64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotGe, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nge_f64_e64" => Ok(DecodedInstruction::VCmpNgeF64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotGe, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ngt_f16_e32" | "v_cmp_ngt_f16" => Ok(DecodedInstruction::VCmpNgtF16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotGt, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ngt_f16_e64" => Ok(DecodedInstruction::VCmpNgtF16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotGt, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ngt_f32_e32" | "v_cmp_ngt_f32" => Ok(DecodedInstruction::VCmpNgtF32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotGt, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ngt_f32_e64" => Ok(DecodedInstruction::VCmpNgtF32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotGt, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ngt_f64_e32" | "v_cmp_ngt_f64" => Ok(DecodedInstruction::VCmpNgtF64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotGt, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_ngt_f64_e64" => Ok(DecodedInstruction::VCmpNgtF64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotGt, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nle_f16_e32" | "v_cmp_nle_f16" => Ok(DecodedInstruction::VCmpNleF16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotLe, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nle_f16_e64" => Ok(DecodedInstruction::VCmpNleF16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotLe, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nle_f32_e32" | "v_cmp_nle_f32" => Ok(DecodedInstruction::VCmpNleF32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotLe, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nle_f32_e64" => Ok(DecodedInstruction::VCmpNleF32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotLe, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nle_f64_e32" | "v_cmp_nle_f64" => Ok(DecodedInstruction::VCmpNleF64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotLe, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nle_f64_e64" => Ok(DecodedInstruction::VCmpNleF64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotLe, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nlg_f16_e32" | "v_cmp_nlg_f16" => Ok(DecodedInstruction::VCmpNlgF16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotLg, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nlg_f16_e64" => Ok(DecodedInstruction::VCmpNlgF16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotLg, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nlg_f32_e32" | "v_cmp_nlg_f32" => Ok(DecodedInstruction::VCmpNlgF32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotLg, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nlg_f32_e64" => Ok(DecodedInstruction::VCmpNlgF32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotLg, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nlg_f64_e32" | "v_cmp_nlg_f64" => Ok(DecodedInstruction::VCmpNlgF64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotLg, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nlg_f64_e64" => Ok(DecodedInstruction::VCmpNlgF64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotLg, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nlt_f16_e32" | "v_cmp_nlt_f16" => Ok(DecodedInstruction::VCmpNltF16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotLt, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nlt_f16_e64" => Ok(DecodedInstruction::VCmpNltF16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotLt, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nlt_f32_e32" | "v_cmp_nlt_f32" => Ok(DecodedInstruction::VCmpNltF32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotLt, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nlt_f32_e64" => Ok(DecodedInstruction::VCmpNltF32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotLt, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nlt_f64_e32" | "v_cmp_nlt_f64" => Ok(DecodedInstruction::VCmpNltF64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotLt, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_nlt_f64_e64" => Ok(DecodedInstruction::VCmpNltF64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::NotLt, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_o_f16_e32" | "v_cmp_o_f16" => Ok(DecodedInstruction::VCmpOF16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ordered, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_o_f16_e64" => Ok(DecodedInstruction::VCmpOF16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ordered, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_o_f32_e32" | "v_cmp_o_f32" => Ok(DecodedInstruction::VCmpOF32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ordered, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_o_f32_e64" => Ok(DecodedInstruction::VCmpOF32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ordered, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_o_f64_e32" | "v_cmp_o_f64" => Ok(DecodedInstruction::VCmpOF64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ordered, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_o_f64_e64" => Ok(DecodedInstruction::VCmpOF64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Ordered, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_t_i32_e32" | "v_cmp_t_i32" => Ok(DecodedInstruction::VCmpTI32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Always, crate::I32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_t_i32_e64" => Ok(DecodedInstruction::VCmpTI32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Always, crate::I32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_t_i64_e32" | "v_cmp_t_i64" => Ok(DecodedInstruction::VCmpTI64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Always, crate::I64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_t_i64_e64" => Ok(DecodedInstruction::VCmpTI64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Always, crate::I64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_t_u32_e32" | "v_cmp_t_u32" => Ok(DecodedInstruction::VCmpTU32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Always, crate::U32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_t_u32_e64" => Ok(DecodedInstruction::VCmpTU32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Always, crate::U32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_t_u64_e32" | "v_cmp_t_u64" => Ok(DecodedInstruction::VCmpTU64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Always, crate::U64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_t_u64_e64" => Ok(DecodedInstruction::VCmpTU64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Always, crate::U64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_u_f16_e32" | "v_cmp_u_f16" => Ok(DecodedInstruction::VCmpUF16E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Unordered, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_u_f16_e64" => Ok(DecodedInstruction::VCmpUF16E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Unordered, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_u_f32_e32" | "v_cmp_u_f32" => Ok(DecodedInstruction::VCmpUF32E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Unordered, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_u_f32_e64" => Ok(DecodedInstruction::VCmpUF32E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Unordered, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_u_f64_e32" | "v_cmp_u_f64" => Ok(DecodedInstruction::VCmpUF64E32({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Unordered, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cmp_u_f64_e64" => Ok(DecodedInstruction::VCmpUF64E64({
            let fields = crate::decode::fields(text, 3)?;
            <crate::VCmp<predicate::Unordered, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0])?,
                crate::decode::operand(&fields[1])?,
                crate::decode::operand(&fields[2])?,
            )
        })),
        "v_cos_f16_e32" | "v_cos_f16" => Ok(DecodedInstruction::VCosF16E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCos, crate::F16, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cos_f16_e64" => Ok(DecodedInstruction::VCosF16E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCos, crate::F16, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cos_f32_e32" | "v_cos_f32" => Ok(DecodedInstruction::VCosF32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCos, crate::F32, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cos_f32_e64" => Ok(DecodedInstruction::VCosF32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCos, crate::F32, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cubeid_f32_e64" => Ok(DecodedInstruction::VCubeidF32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VCubeid,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_cubema_f32_e64" => Ok(DecodedInstruction::VCubemaF32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VCubema,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_cubesc_f32_e64" => Ok(DecodedInstruction::VCubescF32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VCubesc,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_cubetc_f32_e64" => Ok(DecodedInstruction::VCubetcF32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VCubetc,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_cvt_f16_f32_e32" | "v_cvt_f16_f32" => Ok(DecodedInstruction::VCvtF16F32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::F16, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_f16_f32_e64" => Ok(DecodedInstruction::VCvtF16F32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::F16, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_f16_i16_e32" | "v_cvt_f16_i16" => Ok(DecodedInstruction::VCvtF16I16E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::F16, crate::I16, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_f16_i16_e64" => Ok(DecodedInstruction::VCvtF16I16E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::F16, crate::I16, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_f16_u16_e32" | "v_cvt_f16_u16" => Ok(DecodedInstruction::VCvtF16U16E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::F16, crate::U16, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_f16_u16_e64" => Ok(DecodedInstruction::VCvtF16U16E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::F16, crate::U16, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_f32_f16_e32" | "v_cvt_f32_f16" => Ok(DecodedInstruction::VCvtF32F16E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::F32, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_f32_f16_e64" => Ok(DecodedInstruction::VCvtF32F16E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::F32, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_f32_f64_e32" | "v_cvt_f32_f64" => Ok(DecodedInstruction::VCvtF32F64E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::F32, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_f32_f64_e64" => Ok(DecodedInstruction::VCvtF32F64E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::F32, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_f32_i32_e32" | "v_cvt_f32_i32" => Ok(DecodedInstruction::VCvtF32I32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::F32, crate::I32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_f32_i32_e64" => Ok(DecodedInstruction::VCvtF32I32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::F32, crate::I32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_f32_u32_e32" | "v_cvt_f32_u32" => Ok(DecodedInstruction::VCvtF32U32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::F32, crate::U32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_f32_u32_e64" => Ok(DecodedInstruction::VCvtF32U32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::F32, crate::U32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_f64_f32_e32" | "v_cvt_f64_f32" => Ok(DecodedInstruction::VCvtF64F32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::F64, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_f64_f32_e64" => Ok(DecodedInstruction::VCvtF64F32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::F64, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_f64_i32_e32" | "v_cvt_f64_i32" => Ok(DecodedInstruction::VCvtF64I32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::F64, crate::I32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_f64_i32_e64" => Ok(DecodedInstruction::VCvtF64I32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::F64, crate::I32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_f64_u32_e32" | "v_cvt_f64_u32" => Ok(DecodedInstruction::VCvtF64U32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::F64, crate::U32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_f64_u32_e64" => Ok(DecodedInstruction::VCvtF64U32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::F64, crate::U32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_flr_i32_f32_e32" | "v_cvt_flr_i32_f32" => {
            Ok(DecodedInstruction::VCvtFlrI32F32E32({
                let fields = crate::decode::fields(text, 2usize)?;
                <crate::VectorUnary<family::VCvtFlr, crate::I32, crate::F32, crate::E32>>::new(
                    crate::decode::operand(&fields[0usize])?,
                    crate::decode::operand(&fields[1usize])?,
                )
            }))
        }
        "v_cvt_flr_i32_f32_e64" => Ok(DecodedInstruction::VCvtFlrI32F32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvtFlr, crate::I32, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_i16_f16_e32" | "v_cvt_i16_f16" => Ok(DecodedInstruction::VCvtI16F16E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::I16, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_i16_f16_e64" => Ok(DecodedInstruction::VCvtI16F16E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::I16, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_i32_f32_e32" | "v_cvt_i32_f32" => Ok(DecodedInstruction::VCvtI32F32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::I32, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_i32_f32_e64" => Ok(DecodedInstruction::VCvtI32F32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::I32, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_i32_f64_e32" | "v_cvt_i32_f64" => Ok(DecodedInstruction::VCvtI32F64E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::I32, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_i32_f64_e64" => Ok(DecodedInstruction::VCvtI32F64E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::I32, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_norm_i16_f16_e32" | "v_cvt_norm_i16_f16" => {
            Ok(DecodedInstruction::VCvtNormI16F16E32({
                let fields = crate::decode::fields(text, 2usize)?;
                <crate::VectorUnary<family::VCvtNorm, crate::I16, crate::F16, crate::E32>>::new(
                    crate::decode::operand(&fields[0usize])?,
                    crate::decode::operand(&fields[1usize])?,
                )
            }))
        }
        "v_cvt_norm_i16_f16_e64" => Ok(DecodedInstruction::VCvtNormI16F16E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvtNorm, crate::I16, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_norm_u16_f16_e32" | "v_cvt_norm_u16_f16" => {
            Ok(DecodedInstruction::VCvtNormU16F16E32({
                let fields = crate::decode::fields(text, 2usize)?;
                <crate::VectorUnary<family::VCvtNorm, crate::U16, crate::F16, crate::E32>>::new(
                    crate::decode::operand(&fields[0usize])?,
                    crate::decode::operand(&fields[1usize])?,
                )
            }))
        }
        "v_cvt_norm_u16_f16_e64" => Ok(DecodedInstruction::VCvtNormU16F16E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvtNorm, crate::U16, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_rpi_i32_f32_e32" | "v_cvt_rpi_i32_f32" => {
            Ok(DecodedInstruction::VCvtRpiI32F32E32({
                let fields = crate::decode::fields(text, 2usize)?;
                <crate::VectorUnary<family::VCvtRpi, crate::I32, crate::F32, crate::E32>>::new(
                    crate::decode::operand(&fields[0usize])?,
                    crate::decode::operand(&fields[1usize])?,
                )
            }))
        }
        "v_cvt_rpi_i32_f32_e64" => Ok(DecodedInstruction::VCvtRpiI32F32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvtRpi, crate::I32, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_u16_f16_e32" | "v_cvt_u16_f16" => Ok(DecodedInstruction::VCvtU16F16E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::U16, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_u16_f16_e64" => Ok(DecodedInstruction::VCvtU16F16E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::U16, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_u32_f32_e32" | "v_cvt_u32_f32" => Ok(DecodedInstruction::VCvtU32F32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::U32, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_u32_f32_e64" => Ok(DecodedInstruction::VCvtU32F32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::U32, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_u32_f64_e32" | "v_cvt_u32_f64" => Ok(DecodedInstruction::VCvtU32F64E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::U32, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_cvt_u32_f64_e64" => Ok(DecodedInstruction::VCvtU32F64E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VCvt, crate::U32, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_div_fixup_f16_e64" => Ok(DecodedInstruction::VDivFixupF16E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VDivFixup,
                crate::F16,
                crate::F16,
                crate::F16,
                crate::F16,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_div_fixup_f32_e64" => Ok(DecodedInstruction::VDivFixupF32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VDivFixup,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_div_fixup_f64_e64" => Ok(DecodedInstruction::VDivFixupF64E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VDivFixup,
                crate::F64,
                crate::F64,
                crate::F64,
                crate::F64,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_exp_f16_e32" | "v_exp_f16" => Ok(DecodedInstruction::VExpF16E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VExp, crate::F16, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_exp_f16_e64" => Ok(DecodedInstruction::VExpF16E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VExp, crate::F16, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_exp_f32_e32" | "v_exp_f32" => Ok(DecodedInstruction::VExpF32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VExp, crate::F32, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_exp_f32_e64" => Ok(DecodedInstruction::VExpF32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VExp, crate::F32, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_ffbh_i32_e32" | "v_ffbh_i32" => Ok(DecodedInstruction::VFfbhI32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFfbh, crate::I32, crate::I32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_ffbh_i32_e64" => Ok(DecodedInstruction::VFfbhI32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFfbh, crate::I32, crate::I32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_ffbh_u32_e32" | "v_ffbh_u32" => Ok(DecodedInstruction::VFfbhU32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFfbh, crate::I32, crate::U32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_ffbh_u32_e64" => Ok(DecodedInstruction::VFfbhU32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFfbh, crate::I32, crate::U32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_ffbl_b32_e32" | "v_ffbl_b32" => Ok(DecodedInstruction::VFfblB32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFfbl, crate::B32, crate::B32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_ffbl_b32_e64" => Ok(DecodedInstruction::VFfblB32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFfbl, crate::B32, crate::B32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_floor_f16_e32" | "v_floor_f16" => Ok(DecodedInstruction::VFloorF16E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFloor, crate::F16, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_floor_f16_e64" => Ok(DecodedInstruction::VFloorF16E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFloor, crate::F16, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_floor_f32_e32" | "v_floor_f32" => Ok(DecodedInstruction::VFloorF32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFloor, crate::F32, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_floor_f32_e64" => Ok(DecodedInstruction::VFloorF32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFloor, crate::F32, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_floor_f64_e32" | "v_floor_f64" => Ok(DecodedInstruction::VFloorF64E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFloor, crate::F64, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_floor_f64_e64" => Ok(DecodedInstruction::VFloorF64E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFloor, crate::F64, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_fma_f16_e64" => Ok(DecodedInstruction::VFmaF16E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VFma,
                crate::F16,
                crate::F16,
                crate::F16,
                crate::F16,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_fma_f32_e64" => Ok(DecodedInstruction::VFmaF32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VFma,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_fma_f64_e64" => Ok(DecodedInstruction::VFmaF64E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VFma,
                crate::F64,
                crate::F64,
                crate::F64,
                crate::F64,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_fma_legacy_f32_e64" => Ok(DecodedInstruction::VFmaLegacyF32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VFmaLegacy,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_fmac_f16_e32" | "v_fmac_f16" => Ok(DecodedInstruction::VFmacF16E32({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VFmac , crate :: F16 , crate :: F16 , crate :: F16 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_fmac_f16_e64" => Ok(DecodedInstruction::VFmacF16E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VFmac , crate :: F16 , crate :: F16 , crate :: F16 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_fmac_f32_e32" | "v_fmac_f32" => Ok(DecodedInstruction::VFmacF32E32({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VFmac , crate :: F32 , crate :: F32 , crate :: F32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_fmac_f32_e64" => Ok(DecodedInstruction::VFmacF32E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VFmac , crate :: F32 , crate :: F32 , crate :: F32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_fmac_legacy_f32_e32" | "v_fmac_legacy_f32" => {
            Ok(DecodedInstruction::VFmacLegacyF32E32({
                let fields = crate::decode::fields(text, 3usize)?;
                <crate::VectorBinary<
                    family::VFmacLegacy,
                    crate::F32,
                    crate::F32,
                    crate::F32,
                    crate::E32,
                >>::new(
                    crate::decode::operand(&fields[0usize])?,
                    crate::decode::operand(&fields[1usize])?,
                    crate::decode::operand(&fields[2usize])?,
                )
            }))
        }
        "v_fmac_legacy_f32_e64" => Ok(DecodedInstruction::VFmacLegacyF32E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VFmacLegacy , crate :: F32 , crate :: F32 , crate :: F32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_fract_f16_e32" | "v_fract_f16" => Ok(DecodedInstruction::VFractF16E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFract, crate::F16, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_fract_f16_e64" => Ok(DecodedInstruction::VFractF16E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFract, crate::F16, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_fract_f32_e32" | "v_fract_f32" => Ok(DecodedInstruction::VFractF32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFract, crate::F32, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_fract_f32_e64" => Ok(DecodedInstruction::VFractF32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFract, crate::F32, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_fract_f64_e32" | "v_fract_f64" => Ok(DecodedInstruction::VFractF64E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFract, crate::F64, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_fract_f64_e64" => Ok(DecodedInstruction::VFractF64E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFract, crate::F64, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_frexp_exp_i16_f16_e32" | "v_frexp_exp_i16_f16" => {
            Ok(DecodedInstruction::VFrexpExpI16F16E32({
                let fields = crate::decode::fields(text, 2usize)?;
                <crate::VectorUnary<family::VFrexpExp, crate::I16, crate::F16, crate::E32>>::new(
                    crate::decode::operand(&fields[0usize])?,
                    crate::decode::operand(&fields[1usize])?,
                )
            }))
        }
        "v_frexp_exp_i16_f16_e64" => Ok(DecodedInstruction::VFrexpExpI16F16E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFrexpExp, crate::I16, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_frexp_exp_i32_f32_e32" | "v_frexp_exp_i32_f32" => {
            Ok(DecodedInstruction::VFrexpExpI32F32E32({
                let fields = crate::decode::fields(text, 2usize)?;
                <crate::VectorUnary<family::VFrexpExp, crate::I32, crate::F32, crate::E32>>::new(
                    crate::decode::operand(&fields[0usize])?,
                    crate::decode::operand(&fields[1usize])?,
                )
            }))
        }
        "v_frexp_exp_i32_f32_e64" => Ok(DecodedInstruction::VFrexpExpI32F32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFrexpExp, crate::I32, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_frexp_exp_i32_f64_e32" | "v_frexp_exp_i32_f64" => {
            Ok(DecodedInstruction::VFrexpExpI32F64E32({
                let fields = crate::decode::fields(text, 2usize)?;
                <crate::VectorUnary<family::VFrexpExp, crate::I32, crate::F64, crate::E32>>::new(
                    crate::decode::operand(&fields[0usize])?,
                    crate::decode::operand(&fields[1usize])?,
                )
            }))
        }
        "v_frexp_exp_i32_f64_e64" => Ok(DecodedInstruction::VFrexpExpI32F64E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFrexpExp, crate::I32, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_frexp_mant_f16_e32" | "v_frexp_mant_f16" => Ok(DecodedInstruction::VFrexpMantF16E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFrexpMant, crate::F16, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_frexp_mant_f16_e64" => Ok(DecodedInstruction::VFrexpMantF16E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFrexpMant, crate::F16, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_frexp_mant_f32_e32" | "v_frexp_mant_f32" => Ok(DecodedInstruction::VFrexpMantF32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFrexpMant, crate::F32, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_frexp_mant_f32_e64" => Ok(DecodedInstruction::VFrexpMantF32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFrexpMant, crate::F32, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_frexp_mant_f64_e32" | "v_frexp_mant_f64" => Ok(DecodedInstruction::VFrexpMantF64E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFrexpMant, crate::F64, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_frexp_mant_f64_e64" => Ok(DecodedInstruction::VFrexpMantF64E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VFrexpMant, crate::F64, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_ldexp_f16_e32" | "v_ldexp_f16" => Ok(DecodedInstruction::VLdexpF16E32({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VLdexp , crate :: F16 , crate :: F16 , crate :: F16 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_ldexp_f16_e64" => Ok(DecodedInstruction::VLdexpF16E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VLdexp , crate :: F16 , crate :: F16 , crate :: F16 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_ldexp_f32_e64" => Ok(DecodedInstruction::VLdexpF32E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VLdexp , crate :: F32 , crate :: F32 , crate :: I32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_ldexp_f64_e64" => Ok(DecodedInstruction::VLdexpF64E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VLdexp , crate :: F64 , crate :: F64 , crate :: I32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_lerp_u8_e64" => Ok(DecodedInstruction::VLerpU8E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VLerp,
                crate::U32,
                crate::B32,
                crate::B32,
                crate::B32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_log_f16_e32" | "v_log_f16" => Ok(DecodedInstruction::VLogF16E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VLog, crate::F16, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_log_f16_e64" => Ok(DecodedInstruction::VLogF16E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VLog, crate::F16, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_log_f32_e32" | "v_log_f32" => Ok(DecodedInstruction::VLogF32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VLog, crate::F32, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_log_f32_e64" => Ok(DecodedInstruction::VLogF32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VLog, crate::F32, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_lshl_add_u32_e64" => Ok(DecodedInstruction::VLshlAddU32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VLshlAdd,
                crate::U32,
                crate::U32,
                crate::U32,
                crate::U32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_lshl_or_b32_e64" => Ok(DecodedInstruction::VLshlOrB32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VLshlOr,
                crate::B32,
                crate::B32,
                crate::B32,
                crate::B32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_lshlrev_b16_e64" => Ok(DecodedInstruction::VLshlrevB16E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VLshlrev , crate :: B16 , crate :: B16 , crate :: B16 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_lshlrev_b32_e32" | "v_lshlrev_b32" => Ok(DecodedInstruction::VLshlrevB32E32({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VLshlrev , crate :: B32 , crate :: B32 , crate :: B32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_lshlrev_b32_e64" => Ok(DecodedInstruction::VLshlrevB32E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VLshlrev , crate :: B32 , crate :: B32 , crate :: B32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_lshlrev_b64_e64" => Ok(DecodedInstruction::VLshlrevB64E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VLshlrev , crate :: B64 , crate :: U32 , crate :: B64 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_lshrrev_b16_e64" => Ok(DecodedInstruction::VLshrrevB16E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VLshrrev , crate :: B16 , crate :: B16 , crate :: B16 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_lshrrev_b32_e32" | "v_lshrrev_b32" => Ok(DecodedInstruction::VLshrrevB32E32({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VLshrrev , crate :: B32 , crate :: B32 , crate :: B32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_lshrrev_b32_e64" => Ok(DecodedInstruction::VLshrrevB32E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VLshrrev , crate :: B32 , crate :: B32 , crate :: B32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_lshrrev_b64_e64" => Ok(DecodedInstruction::VLshrrevB64E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VLshrrev , crate :: B64 , crate :: U32 , crate :: B64 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_mad_i16_e64" => Ok(DecodedInstruction::VMadI16E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMad,
                crate::I16,
                crate::I16,
                crate::I16,
                crate::I16,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_mad_i32_i16_e64" => Ok(DecodedInstruction::VMadI32I16E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMad,
                crate::I32,
                crate::I16,
                crate::I16,
                crate::I32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_mad_u16_e64" => Ok(DecodedInstruction::VMadU16E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMad,
                crate::U16,
                crate::U16,
                crate::U16,
                crate::U16,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_mad_u32_u16_e64" => Ok(DecodedInstruction::VMadU32U16E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMad,
                crate::U32,
                crate::U16,
                crate::U16,
                crate::U32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_max3_f16_e64" => Ok(DecodedInstruction::VMax3F16E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMax3,
                crate::F16,
                crate::F16,
                crate::F16,
                crate::F16,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_max3_f32_e64" => Ok(DecodedInstruction::VMax3F32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMax3,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_max3_i16_e64" => Ok(DecodedInstruction::VMax3I16E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMax3,
                crate::I16,
                crate::I16,
                crate::I16,
                crate::I16,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_max3_i32_e64" => Ok(DecodedInstruction::VMax3I32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMax3,
                crate::I32,
                crate::I32,
                crate::I32,
                crate::I32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_max3_u16_e64" => Ok(DecodedInstruction::VMax3U16E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMax3,
                crate::U16,
                crate::U16,
                crate::U16,
                crate::U16,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_max3_u32_e64" => Ok(DecodedInstruction::VMax3U32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMax3,
                crate::U32,
                crate::U32,
                crate::U32,
                crate::U32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_max_f16_e32" | "v_max_f16" => {
            Ok(DecodedInstruction::VMaxF16E32({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMax , crate :: F16 , crate :: F16 , crate :: F16 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_max_f16_e64" => {
            Ok(DecodedInstruction::VMaxF16E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMax , crate :: F16 , crate :: F16 , crate :: F16 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_max_f32_e32" | "v_max_f32" => {
            Ok(DecodedInstruction::VMaxF32E32({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMax , crate :: F32 , crate :: F32 , crate :: F32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_max_f32_e64" => {
            Ok(DecodedInstruction::VMaxF32E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMax , crate :: F32 , crate :: F32 , crate :: F32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_max_f64_e64" => {
            Ok(DecodedInstruction::VMaxF64E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMax , crate :: F64 , crate :: F64 , crate :: F64 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_max_i16_e64" => {
            Ok(DecodedInstruction::VMaxI16E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMax , crate :: I16 , crate :: I16 , crate :: I16 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_max_i32_e32" | "v_max_i32" => {
            Ok(DecodedInstruction::VMaxI32E32({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMax , crate :: I32 , crate :: I32 , crate :: I32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_max_i32_e64" => {
            Ok(DecodedInstruction::VMaxI32E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMax , crate :: I32 , crate :: I32 , crate :: I32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_max_u16_e64" => {
            Ok(DecodedInstruction::VMaxU16E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMax , crate :: U16 , crate :: U16 , crate :: U16 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_max_u32_e32" | "v_max_u32" => {
            Ok(DecodedInstruction::VMaxU32E32({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMax , crate :: U32 , crate :: U32 , crate :: U32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_max_u32_e64" => {
            Ok(DecodedInstruction::VMaxU32E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMax , crate :: U32 , crate :: U32 , crate :: U32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_mbcnt_hi_u32_b32_e64" => Ok(DecodedInstruction::VMbcntHiU32B32E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VMbcntHi , crate :: B32 , crate :: B32 , crate :: B32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_mbcnt_lo_u32_b32_e64" => Ok(DecodedInstruction::VMbcntLoU32B32E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VMbcntLo , crate :: B32 , crate :: B32 , crate :: B32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_med3_f16_e64" => Ok(DecodedInstruction::VMed3F16E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMed3,
                crate::F16,
                crate::F16,
                crate::F16,
                crate::F16,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_med3_f32_e64" => Ok(DecodedInstruction::VMed3F32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMed3,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_med3_i16_e64" => Ok(DecodedInstruction::VMed3I16E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMed3,
                crate::I16,
                crate::I16,
                crate::I16,
                crate::I16,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_med3_i32_e64" => Ok(DecodedInstruction::VMed3I32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMed3,
                crate::I32,
                crate::I32,
                crate::I32,
                crate::I32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_med3_u16_e64" => Ok(DecodedInstruction::VMed3U16E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMed3,
                crate::U16,
                crate::U16,
                crate::U16,
                crate::U16,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_med3_u32_e64" => Ok(DecodedInstruction::VMed3U32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMed3,
                crate::U32,
                crate::U32,
                crate::U32,
                crate::U32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_min3_f16_e64" => Ok(DecodedInstruction::VMin3F16E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMin3,
                crate::F16,
                crate::F16,
                crate::F16,
                crate::F16,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_min3_f32_e64" => Ok(DecodedInstruction::VMin3F32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMin3,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_min3_i16_e64" => Ok(DecodedInstruction::VMin3I16E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMin3,
                crate::I16,
                crate::I16,
                crate::I16,
                crate::I16,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_min3_i32_e64" => Ok(DecodedInstruction::VMin3I32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMin3,
                crate::I32,
                crate::I32,
                crate::I32,
                crate::I32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_min3_u16_e64" => Ok(DecodedInstruction::VMin3U16E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMin3,
                crate::U16,
                crate::U16,
                crate::U16,
                crate::U16,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_min3_u32_e64" => Ok(DecodedInstruction::VMin3U32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMin3,
                crate::U32,
                crate::U32,
                crate::U32,
                crate::U32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_min_f16_e32" | "v_min_f16" => {
            Ok(DecodedInstruction::VMinF16E32({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMin , crate :: F16 , crate :: F16 , crate :: F16 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_min_f16_e64" => {
            Ok(DecodedInstruction::VMinF16E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMin , crate :: F16 , crate :: F16 , crate :: F16 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_min_f32_e32" | "v_min_f32" => {
            Ok(DecodedInstruction::VMinF32E32({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMin , crate :: F32 , crate :: F32 , crate :: F32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_min_f32_e64" => {
            Ok(DecodedInstruction::VMinF32E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMin , crate :: F32 , crate :: F32 , crate :: F32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_min_f64_e64" => {
            Ok(DecodedInstruction::VMinF64E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMin , crate :: F64 , crate :: F64 , crate :: F64 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_min_i16_e64" => {
            Ok(DecodedInstruction::VMinI16E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMin , crate :: I16 , crate :: I16 , crate :: I16 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_min_i32_e32" | "v_min_i32" => {
            Ok(DecodedInstruction::VMinI32E32({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMin , crate :: I32 , crate :: I32 , crate :: I32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_min_i32_e64" => {
            Ok(DecodedInstruction::VMinI32E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMin , crate :: I32 , crate :: I32 , crate :: I32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_min_u16_e64" => {
            Ok(DecodedInstruction::VMinU16E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMin , crate :: U16 , crate :: U16 , crate :: U16 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_min_u32_e32" | "v_min_u32" => {
            Ok(DecodedInstruction::VMinU32E32({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMin , crate :: U32 , crate :: U32 , crate :: U32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_min_u32_e64" => {
            Ok(DecodedInstruction::VMinU32E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMin , crate :: U32 , crate :: U32 , crate :: U32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_mov_b32_e32" | "v_mov_b32" => Ok(DecodedInstruction::VMovB32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VMov, crate::B32, crate::B32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_mov_b32_e64" => Ok(DecodedInstruction::VMovB32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VMov, crate::B32, crate::B32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_mqsad_pk_u16_u8_e64" => Ok(DecodedInstruction::VMqsadPkU16U8E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMqsadPk,
                crate::B64,
                crate::B64,
                crate::B32,
                crate::B64,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_msad_u8_e64" => Ok(DecodedInstruction::VMsadU8E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMsad,
                crate::U32,
                crate::B32,
                crate::B32,
                crate::B32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_mul_f16_e32" | "v_mul_f16" => {
            Ok(DecodedInstruction::VMulF16E32({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMul , crate :: F16 , crate :: F16 , crate :: F16 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_mul_f16_e64" => {
            Ok(DecodedInstruction::VMulF16E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMul , crate :: F16 , crate :: F16 , crate :: F16 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_mul_f32_e32" | "v_mul_f32" => {
            Ok(DecodedInstruction::VMulF32E32({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMul , crate :: F32 , crate :: F32 , crate :: F32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_mul_f32_e64" => {
            Ok(DecodedInstruction::VMulF32E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMul , crate :: F32 , crate :: F32 , crate :: F32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_mul_f64_e64" => {
            Ok(DecodedInstruction::VMulF64E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VMul , crate :: F64 , crate :: F64 , crate :: F64 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_mul_hi_i32_e64" => Ok(DecodedInstruction::VMulHiI32E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VMulHi , crate :: I32 , crate :: I32 , crate :: I32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_mul_hi_u32_e64" => Ok(DecodedInstruction::VMulHiU32E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VMulHi , crate :: U32 , crate :: U32 , crate :: U32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_mul_legacy_f32_e32" | "v_mul_legacy_f32" => Ok(DecodedInstruction::VMulLegacyF32E32({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VMulLegacy , crate :: F32 , crate :: F32 , crate :: F32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_mul_legacy_f32_e64" => Ok(DecodedInstruction::VMulLegacyF32E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VMulLegacy , crate :: F32 , crate :: F32 , crate :: F32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_mul_lo_u16_e64" => Ok(DecodedInstruction::VMulLoU16E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VMulLo , crate :: U16 , crate :: U16 , crate :: U16 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_mul_lo_u32_e64" => Ok(DecodedInstruction::VMulLoU32E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VMulLo , crate :: U32 , crate :: U32 , crate :: U32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_mullit_f32_e64" => Ok(DecodedInstruction::VMullitF32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VMullit,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::F32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_not_b32_e32" | "v_not_b32" => Ok(DecodedInstruction::VNotB32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VNot, crate::B32, crate::B32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_not_b32_e64" => Ok(DecodedInstruction::VNotB32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VNot, crate::B32, crate::B32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_or3_b32_e64" => Ok(DecodedInstruction::VOr3B32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VOr3,
                crate::B32,
                crate::B32,
                crate::B32,
                crate::B32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_or_b32_e32" | "v_or_b32" => {
            Ok(DecodedInstruction::VOrB32E32({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VOr , crate :: B32 , crate :: B32 , crate :: B32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_or_b32_e64" => {
            Ok(DecodedInstruction::VOrB32E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VOr , crate :: B32 , crate :: B32 , crate :: B32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_pack_b32_f16_e64" => Ok(DecodedInstruction::VPackB32F16E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VPack , crate :: B32 , crate :: F16 , crate :: F16 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_qsad_pk_u16_u8_e64" => Ok(DecodedInstruction::VQsadPkU16U8E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VQsadPk,
                crate::B64,
                crate::B64,
                crate::B32,
                crate::B64,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_rcp_f16_e32" | "v_rcp_f16" => Ok(DecodedInstruction::VRcpF16E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VRcp, crate::F16, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_rcp_f16_e64" => Ok(DecodedInstruction::VRcpF16E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VRcp, crate::F16, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_rcp_f32_e32" | "v_rcp_f32" => Ok(DecodedInstruction::VRcpF32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VRcp, crate::F32, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_rcp_f32_e64" => Ok(DecodedInstruction::VRcpF32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VRcp, crate::F32, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_rcp_f64_e32" | "v_rcp_f64" => Ok(DecodedInstruction::VRcpF64E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VRcp, crate::F64, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_rcp_f64_e64" => Ok(DecodedInstruction::VRcpF64E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VRcp, crate::F64, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_rcp_iflag_f32_e32" | "v_rcp_iflag_f32" => Ok(DecodedInstruction::VRcpIflagF32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VRcpIflag, crate::F32, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_rcp_iflag_f32_e64" => Ok(DecodedInstruction::VRcpIflagF32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VRcpIflag, crate::F32, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_rndne_f16_e32" | "v_rndne_f16" => Ok(DecodedInstruction::VRndneF16E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VRndne, crate::F16, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_rndne_f16_e64" => Ok(DecodedInstruction::VRndneF16E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VRndne, crate::F16, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_rndne_f32_e32" | "v_rndne_f32" => Ok(DecodedInstruction::VRndneF32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VRndne, crate::F32, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_rndne_f32_e64" => Ok(DecodedInstruction::VRndneF32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VRndne, crate::F32, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_rndne_f64_e32" | "v_rndne_f64" => Ok(DecodedInstruction::VRndneF64E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VRndne, crate::F64, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_rndne_f64_e64" => Ok(DecodedInstruction::VRndneF64E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VRndne, crate::F64, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_rsq_f16_e32" | "v_rsq_f16" => Ok(DecodedInstruction::VRsqF16E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VRsq, crate::F16, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_rsq_f16_e64" => Ok(DecodedInstruction::VRsqF16E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VRsq, crate::F16, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_rsq_f32_e32" | "v_rsq_f32" => Ok(DecodedInstruction::VRsqF32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VRsq, crate::F32, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_rsq_f32_e64" => Ok(DecodedInstruction::VRsqF32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VRsq, crate::F32, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_rsq_f64_e32" | "v_rsq_f64" => Ok(DecodedInstruction::VRsqF64E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VRsq, crate::F64, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_rsq_f64_e64" => Ok(DecodedInstruction::VRsqF64E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VRsq, crate::F64, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_sad_u32_e64" => Ok(DecodedInstruction::VSadU32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VSad,
                crate::U32,
                crate::U32,
                crate::U32,
                crate::U32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_sin_f16_e32" | "v_sin_f16" => Ok(DecodedInstruction::VSinF16E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VSin, crate::F16, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_sin_f16_e64" => Ok(DecodedInstruction::VSinF16E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VSin, crate::F16, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_sin_f32_e32" | "v_sin_f32" => Ok(DecodedInstruction::VSinF32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VSin, crate::F32, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_sin_f32_e64" => Ok(DecodedInstruction::VSinF32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VSin, crate::F32, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_sqrt_f16_e32" | "v_sqrt_f16" => Ok(DecodedInstruction::VSqrtF16E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VSqrt, crate::F16, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_sqrt_f16_e64" => Ok(DecodedInstruction::VSqrtF16E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VSqrt, crate::F16, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_sqrt_f32_e32" | "v_sqrt_f32" => Ok(DecodedInstruction::VSqrtF32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VSqrt, crate::F32, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_sqrt_f32_e64" => Ok(DecodedInstruction::VSqrtF32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VSqrt, crate::F32, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_sqrt_f64_e32" | "v_sqrt_f64" => Ok(DecodedInstruction::VSqrtF64E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VSqrt, crate::F64, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_sqrt_f64_e64" => Ok(DecodedInstruction::VSqrtF64E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VSqrt, crate::F64, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_sub_f16_e32" | "v_sub_f16" => {
            Ok(DecodedInstruction::VSubF16E32({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VSub , crate :: F16 , crate :: F16 , crate :: F16 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_sub_f16_e64" => {
            Ok(DecodedInstruction::VSubF16E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VSub , crate :: F16 , crate :: F16 , crate :: F16 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_sub_f32_e32" | "v_sub_f32" => {
            Ok(DecodedInstruction::VSubF32E32({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VSub , crate :: F32 , crate :: F32 , crate :: F32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_sub_f32_e64" => {
            Ok(DecodedInstruction::VSubF32E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VSub , crate :: F32 , crate :: F32 , crate :: F32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_sub_nc_i16_e64" => Ok(DecodedInstruction::VSubNcI16E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VSubNc , crate :: I16 , crate :: I16 , crate :: I16 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_sub_nc_i32_e64" => Ok(DecodedInstruction::VSubNcI32E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VSubNc , crate :: I32 , crate :: I32 , crate :: I32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_sub_nc_u16_e64" => Ok(DecodedInstruction::VSubNcU16E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VSubNc , crate :: U16 , crate :: U16 , crate :: U16 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_sub_nc_u32_e32" | "v_sub_nc_u32" => Ok(DecodedInstruction::VSubNcU32E32({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VSubNc , crate :: U32 , crate :: U32 , crate :: U32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_sub_nc_u32_e64" => Ok(DecodedInstruction::VSubNcU32E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VSubNc , crate :: U32 , crate :: U32 , crate :: U32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_subrev_f16_e32" | "v_subrev_f16" => Ok(DecodedInstruction::VSubrevF16E32({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VSubrev , crate :: F16 , crate :: F16 , crate :: F16 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_subrev_f16_e64" => Ok(DecodedInstruction::VSubrevF16E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VSubrev , crate :: F16 , crate :: F16 , crate :: F16 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_subrev_f32_e32" | "v_subrev_f32" => Ok(DecodedInstruction::VSubrevF32E32({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VSubrev , crate :: F32 , crate :: F32 , crate :: F32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_subrev_f32_e64" => Ok(DecodedInstruction::VSubrevF32E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VSubrev , crate :: F32 , crate :: F32 , crate :: F32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_subrev_nc_u32_e32" | "v_subrev_nc_u32" => Ok(DecodedInstruction::VSubrevNcU32E32({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VSubrevNc , crate :: U32 , crate :: U32 , crate :: U32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_subrev_nc_u32_e64" => Ok(DecodedInstruction::VSubrevNcU32E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VSubrevNc , crate :: U32 , crate :: U32 , crate :: U32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_trig_preop_f64_e64" => Ok(DecodedInstruction::VTrigPreopF64E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VTrigPreop , crate :: F64 , crate :: F64 , crate :: B32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_trunc_f16_e32" | "v_trunc_f16" => Ok(DecodedInstruction::VTruncF16E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VTrunc, crate::F16, crate::F16, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_trunc_f16_e64" => Ok(DecodedInstruction::VTruncF16E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VTrunc, crate::F16, crate::F16, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_trunc_f32_e32" | "v_trunc_f32" => Ok(DecodedInstruction::VTruncF32E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VTrunc, crate::F32, crate::F32, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_trunc_f32_e64" => Ok(DecodedInstruction::VTruncF32E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VTrunc, crate::F32, crate::F32, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_trunc_f64_e32" | "v_trunc_f64" => Ok(DecodedInstruction::VTruncF64E32({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VTrunc, crate::F64, crate::F64, crate::E32>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_trunc_f64_e64" => Ok(DecodedInstruction::VTruncF64E64({
            let fields = crate::decode::fields(text, 2usize)?;
            <crate::VectorUnary<family::VTrunc, crate::F64, crate::F64, crate::E64>>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
            )
        })),
        "v_xad_u32_e64" => Ok(DecodedInstruction::VXadU32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VXad,
                crate::U32,
                crate::U32,
                crate::U32,
                crate::U32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_xnor_b32_e32" | "v_xnor_b32" => Ok(DecodedInstruction::VXnorB32E32({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VXnor , crate :: B32 , crate :: B32 , crate :: B32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_xnor_b32_e64" => Ok(DecodedInstruction::VXnorB32E64({
            let fields = crate::decode::fields(text, 3usize)?;
            < crate :: VectorBinary < family :: VXnor , crate :: B32 , crate :: B32 , crate :: B32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
        })),
        "v_xor3_b32_e64" => Ok(DecodedInstruction::VXor3B32E64({
            let fields = crate::decode::fields(text, 4usize)?;
            <crate::VectorTernary<
                family::VXor3,
                crate::B32,
                crate::B32,
                crate::B32,
                crate::B32,
                crate::E64,
            >>::new(
                crate::decode::operand(&fields[0usize])?,
                crate::decode::operand(&fields[1usize])?,
                crate::decode::operand(&fields[2usize])?,
                crate::decode::operand(&fields[3usize])?,
            )
        })),
        "v_xor_b32_e32" | "v_xor_b32" => {
            Ok(DecodedInstruction::VXorB32E32({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VXor , crate :: B32 , crate :: B32 , crate :: B32 , crate :: E32 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        "v_xor_b32_e64" => {
            Ok(DecodedInstruction::VXorB32E64({
                let fields = crate::decode::fields(text, 3usize)?;
                < crate :: VectorBinary < family :: VXor , crate :: B32 , crate :: B32 , crate :: B32 , crate :: E64 > > :: new (crate :: decode :: operand (& fields [0usize]) ? , crate :: decode :: operand (& fields [1usize]) ? , crate :: decode :: operand (& fields [2usize]) ? ,)
            }))
        }
        _ => Err(crate::DecodeError {
            range: 0..text.split_whitespace().next().unwrap_or("").len(),
            reason: "instruction has no generated decoder".into(),
        }),
    }
}
pub const KNOWN_MNEMONICS: &[&str] = &[
    "buffer_atomic_add",
    "buffer_atomic_add_x2",
    "buffer_atomic_and",
    "buffer_atomic_and_x2",
    "buffer_atomic_cmpswap",
    "buffer_atomic_cmpswap_x2",
    "buffer_atomic_csub",
    "buffer_atomic_dec",
    "buffer_atomic_dec_x2",
    "buffer_atomic_fcmpswap",
    "buffer_atomic_fcmpswap_x2",
    "buffer_atomic_fmax",
    "buffer_atomic_fmax_x2",
    "buffer_atomic_fmin",
    "buffer_atomic_fmin_x2",
    "buffer_atomic_inc",
    "buffer_atomic_inc_x2",
    "buffer_atomic_or",
    "buffer_atomic_or_x2",
    "buffer_atomic_smax",
    "buffer_atomic_smax_x2",
    "buffer_atomic_smin",
    "buffer_atomic_smin_x2",
    "buffer_atomic_sub",
    "buffer_atomic_sub_x2",
    "buffer_atomic_swap",
    "buffer_atomic_swap_x2",
    "buffer_atomic_umax",
    "buffer_atomic_umax_x2",
    "buffer_atomic_umin",
    "buffer_atomic_umin_x2",
    "buffer_atomic_xor",
    "buffer_atomic_xor_x2",
    "buffer_gl0_inv",
    "buffer_gl1_inv",
    "buffer_load_dword",
    "buffer_load_dwordx2",
    "buffer_load_dwordx3",
    "buffer_load_dwordx4",
    "buffer_load_format_d16_hi_x",
    "buffer_load_format_d16_x",
    "buffer_load_format_d16_xy",
    "buffer_load_format_d16_xyz",
    "buffer_load_format_d16_xyzw",
    "buffer_load_format_x",
    "buffer_load_format_xy",
    "buffer_load_format_xyz",
    "buffer_load_format_xyzw",
    "buffer_load_sbyte",
    "buffer_load_sbyte_d16",
    "buffer_load_sbyte_d16_hi",
    "buffer_load_short_d16",
    "buffer_load_short_d16_hi",
    "buffer_load_sshort",
    "buffer_load_ubyte",
    "buffer_load_ubyte_d16",
    "buffer_load_ubyte_d16_hi",
    "buffer_load_ushort",
    "buffer_store_byte",
    "buffer_store_byte_d16_hi",
    "buffer_store_dword",
    "buffer_store_dwordx2",
    "buffer_store_dwordx3",
    "buffer_store_dwordx4",
    "buffer_store_format_d16_hi_x",
    "buffer_store_format_d16_x",
    "buffer_store_format_d16_xy",
    "buffer_store_format_d16_xyz",
    "buffer_store_format_d16_xyzw",
    "buffer_store_format_x",
    "buffer_store_format_xy",
    "buffer_store_format_xyz",
    "buffer_store_format_xyzw",
    "buffer_store_short",
    "buffer_store_short_d16_hi",
    "ds_add_f32",
    "ds_add_rtn_f32",
    "ds_add_rtn_u32",
    "ds_add_rtn_u64",
    "ds_add_u32",
    "ds_add_u64",
    "ds_and_b32",
    "ds_and_b64",
    "ds_and_rtn_b32",
    "ds_and_rtn_b64",
    "ds_append",
    "ds_bpermute_b32",
    "ds_cmpst_b32",
    "ds_cmpst_b64",
    "ds_cmpst_f32",
    "ds_cmpst_f64",
    "ds_cmpst_rtn_b32",
    "ds_cmpst_rtn_b64",
    "ds_cmpst_rtn_f32",
    "ds_cmpst_rtn_f64",
    "ds_condxchg32_rtn_b64",
    "ds_consume",
    "ds_dec_rtn_u32",
    "ds_dec_rtn_u64",
    "ds_dec_u32",
    "ds_dec_u64",
    "ds_gws_barrier",
    "ds_gws_init",
    "ds_gws_sema_br",
    "ds_gws_sema_p",
    "ds_gws_sema_release_all",
    "ds_gws_sema_v",
    "ds_inc_rtn_u32",
    "ds_inc_rtn_u64",
    "ds_inc_u32",
    "ds_inc_u64",
    "ds_max_f32",
    "ds_max_f64",
    "ds_max_i32",
    "ds_max_i64",
    "ds_max_rtn_f32",
    "ds_max_rtn_f64",
    "ds_max_rtn_i32",
    "ds_max_rtn_i64",
    "ds_max_rtn_u32",
    "ds_max_rtn_u64",
    "ds_max_u32",
    "ds_max_u64",
    "ds_min_f32",
    "ds_min_f64",
    "ds_min_i32",
    "ds_min_i64",
    "ds_min_rtn_f32",
    "ds_min_rtn_f64",
    "ds_min_rtn_i32",
    "ds_min_rtn_i64",
    "ds_min_rtn_u32",
    "ds_min_rtn_u64",
    "ds_min_u32",
    "ds_min_u64",
    "ds_mskor_b32",
    "ds_mskor_b64",
    "ds_mskor_rtn_b32",
    "ds_mskor_rtn_b64",
    "ds_nop",
    "ds_or_b32",
    "ds_or_b64",
    "ds_or_rtn_b32",
    "ds_or_rtn_b64",
    "ds_ordered_count",
    "ds_permute_b32",
    "ds_read2_b32",
    "ds_read2_b64",
    "ds_read2st64_b32",
    "ds_read2st64_b64",
    "ds_read_addtid_b32",
    "ds_read_b128",
    "ds_read_b32",
    "ds_read_b64",
    "ds_read_b96",
    "ds_read_i16",
    "ds_read_i8",
    "ds_read_i8_d16",
    "ds_read_i8_d16_hi",
    "ds_read_u16",
    "ds_read_u16_d16",
    "ds_read_u16_d16_hi",
    "ds_read_u8",
    "ds_read_u8_d16",
    "ds_read_u8_d16_hi",
    "ds_rsub_rtn_u32",
    "ds_rsub_rtn_u64",
    "ds_rsub_u32",
    "ds_rsub_u64",
    "ds_sub_rtn_u32",
    "ds_sub_rtn_u64",
    "ds_sub_u32",
    "ds_sub_u64",
    "ds_swizzle_b32",
    "ds_wrap_rtn_b32",
    "ds_write2_b32",
    "ds_write2_b64",
    "ds_write2st64_b32",
    "ds_write2st64_b64",
    "ds_write_addtid_b32",
    "ds_write_b128",
    "ds_write_b16",
    "ds_write_b16_d16_hi",
    "ds_write_b32",
    "ds_write_b64",
    "ds_write_b8",
    "ds_write_b8_d16_hi",
    "ds_write_b96",
    "ds_wrxchg2_rtn_b32",
    "ds_wrxchg2_rtn_b64",
    "ds_wrxchg2st64_rtn_b32",
    "ds_wrxchg2st64_rtn_b64",
    "ds_wrxchg_rtn_b32",
    "ds_wrxchg_rtn_b64",
    "ds_xor_b32",
    "ds_xor_b64",
    "ds_xor_rtn_b32",
    "ds_xor_rtn_b64",
    "exp",
    "flat_atomic_add",
    "flat_atomic_add_x2",
    "flat_atomic_and",
    "flat_atomic_and_x2",
    "flat_atomic_cmpswap",
    "flat_atomic_cmpswap_x2",
    "flat_atomic_dec",
    "flat_atomic_dec_x2",
    "flat_atomic_fcmpswap",
    "flat_atomic_fcmpswap_x2",
    "flat_atomic_fmax",
    "flat_atomic_fmax_x2",
    "flat_atomic_fmin",
    "flat_atomic_fmin_x2",
    "flat_atomic_inc",
    "flat_atomic_inc_x2",
    "flat_atomic_or",
    "flat_atomic_or_x2",
    "flat_atomic_smax",
    "flat_atomic_smax_x2",
    "flat_atomic_smin",
    "flat_atomic_smin_x2",
    "flat_atomic_sub",
    "flat_atomic_sub_x2",
    "flat_atomic_swap",
    "flat_atomic_swap_x2",
    "flat_atomic_umax",
    "flat_atomic_umax_x2",
    "flat_atomic_umin",
    "flat_atomic_umin_x2",
    "flat_atomic_xor",
    "flat_atomic_xor_x2",
    "flat_load_dword",
    "flat_load_dwordx2",
    "flat_load_dwordx3",
    "flat_load_dwordx4",
    "flat_load_sbyte",
    "flat_load_sbyte_d16",
    "flat_load_sbyte_d16_hi",
    "flat_load_short_d16",
    "flat_load_short_d16_hi",
    "flat_load_sshort",
    "flat_load_ubyte",
    "flat_load_ubyte_d16",
    "flat_load_ubyte_d16_hi",
    "flat_load_ushort",
    "flat_store_byte",
    "flat_store_byte_d16_hi",
    "flat_store_dword",
    "flat_store_dwordx2",
    "flat_store_dwordx3",
    "flat_store_dwordx4",
    "flat_store_short",
    "flat_store_short_d16_hi",
    "global_atomic_add",
    "global_atomic_add_x2",
    "global_atomic_and",
    "global_atomic_and_x2",
    "global_atomic_cmpswap",
    "global_atomic_cmpswap_x2",
    "global_atomic_csub",
    "global_atomic_dec",
    "global_atomic_dec_x2",
    "global_atomic_fcmpswap",
    "global_atomic_fcmpswap_x2",
    "global_atomic_fmax",
    "global_atomic_fmax_x2",
    "global_atomic_fmin",
    "global_atomic_fmin_x2",
    "global_atomic_inc",
    "global_atomic_inc_x2",
    "global_atomic_or",
    "global_atomic_or_x2",
    "global_atomic_smax",
    "global_atomic_smax_x2",
    "global_atomic_smin",
    "global_atomic_smin_x2",
    "global_atomic_sub",
    "global_atomic_sub_x2",
    "global_atomic_swap",
    "global_atomic_swap_x2",
    "global_atomic_umax",
    "global_atomic_umax_x2",
    "global_atomic_umin",
    "global_atomic_umin_x2",
    "global_atomic_xor",
    "global_atomic_xor_x2",
    "global_load_dword",
    "global_load_dword_addtid",
    "global_load_dwordx2",
    "global_load_dwordx3",
    "global_load_dwordx4",
    "global_load_sbyte",
    "global_load_sbyte_d16",
    "global_load_sbyte_d16_hi",
    "global_load_short_d16",
    "global_load_short_d16_hi",
    "global_load_sshort",
    "global_load_ubyte",
    "global_load_ubyte_d16",
    "global_load_ubyte_d16_hi",
    "global_load_ushort",
    "global_store_byte",
    "global_store_byte_d16_hi",
    "global_store_dword",
    "global_store_dword_addtid",
    "global_store_dwordx2",
    "global_store_dwordx3",
    "global_store_dwordx4",
    "global_store_short",
    "global_store_short_d16_hi",
    "image_atomic_add",
    "image_atomic_and",
    "image_atomic_cmpswap",
    "image_atomic_dec",
    "image_atomic_fcmpswap",
    "image_atomic_fmax",
    "image_atomic_fmin",
    "image_atomic_inc",
    "image_atomic_or",
    "image_atomic_smax",
    "image_atomic_smin",
    "image_atomic_sub",
    "image_atomic_swap",
    "image_atomic_umax",
    "image_atomic_umin",
    "image_atomic_xor",
    "image_bvh64_intersect_ray",
    "image_bvh_intersect_ray",
    "image_gather4",
    "image_gather4_b",
    "image_gather4_b_cl",
    "image_gather4_b_cl_o",
    "image_gather4_b_o",
    "image_gather4_c",
    "image_gather4_c_b",
    "image_gather4_c_b_cl",
    "image_gather4_c_b_cl_o",
    "image_gather4_c_b_o",
    "image_gather4_c_cl",
    "image_gather4_c_cl_o",
    "image_gather4_c_l",
    "image_gather4_c_l_o",
    "image_gather4_c_lz",
    "image_gather4_c_lz_o",
    "image_gather4_c_o",
    "image_gather4_cl",
    "image_gather4_cl_o",
    "image_gather4_l",
    "image_gather4_l_o",
    "image_gather4_lz",
    "image_gather4_lz_o",
    "image_gather4_o",
    "image_gather4h",
    "image_gather4h_pck",
    "image_gather8h_pck",
    "image_get_lod",
    "image_get_resinfo",
    "image_load",
    "image_load_by2",
    "image_load_by4",
    "image_load_mip",
    "image_load_mip_by2",
    "image_load_mip_by4",
    "image_load_mip_pck",
    "image_load_mip_pck2",
    "image_load_mip_pck4",
    "image_load_mip_pck_sgn",
    "image_load_pck",
    "image_load_pck2",
    "image_load_pck4",
    "image_load_pck_sgn",
    "image_msaa_load",
    "image_sample",
    "image_sample_b",
    "image_sample_b_cl",
    "image_sample_b_cl_o",
    "image_sample_b_o",
    "image_sample_c",
    "image_sample_c_b",
    "image_sample_c_b_cl",
    "image_sample_c_b_cl_o",
    "image_sample_c_b_o",
    "image_sample_c_cd",
    "image_sample_c_cd_cl",
    "image_sample_c_cd_cl_g16",
    "image_sample_c_cd_cl_o",
    "image_sample_c_cd_cl_o_g16",
    "image_sample_c_cd_g16",
    "image_sample_c_cd_o",
    "image_sample_c_cd_o_g16",
    "image_sample_c_cl",
    "image_sample_c_cl_o",
    "image_sample_c_d",
    "image_sample_c_d_cl",
    "image_sample_c_d_cl_g16",
    "image_sample_c_d_cl_o",
    "image_sample_c_d_cl_o_g16",
    "image_sample_c_d_g16",
    "image_sample_c_d_o",
    "image_sample_c_d_o_g16",
    "image_sample_c_l",
    "image_sample_c_l_o",
    "image_sample_c_lz",
    "image_sample_c_lz_o",
    "image_sample_c_o",
    "image_sample_cd",
    "image_sample_cd_cl",
    "image_sample_cd_cl_g16",
    "image_sample_cd_cl_o",
    "image_sample_cd_cl_o_g16",
    "image_sample_cd_g16",
    "image_sample_cd_o",
    "image_sample_cd_o_g16",
    "image_sample_cl",
    "image_sample_cl_o",
    "image_sample_d",
    "image_sample_d_cl",
    "image_sample_d_cl_g16",
    "image_sample_d_cl_o",
    "image_sample_d_cl_o_g16",
    "image_sample_d_g16",
    "image_sample_d_o",
    "image_sample_d_o_g16",
    "image_sample_l",
    "image_sample_l_o",
    "image_sample_lz",
    "image_sample_lz_o",
    "image_sample_o",
    "image_store",
    "image_store_by2",
    "image_store_by4",
    "image_store_mip",
    "image_store_mip_by2",
    "image_store_mip_by4",
    "image_store_mip_pck",
    "image_store_mip_pck2",
    "image_store_mip_pck4",
    "image_store_pck",
    "image_store_pck2",
    "image_store_pck4",
    "s_abs_i32",
    "s_absdiff_i32",
    "s_add_i32",
    "s_add_u32",
    "s_addc_u32",
    "s_addk_i32",
    "s_and_b32",
    "s_and_b64",
    "s_and_saveexec_b32",
    "s_and_saveexec_b64",
    "s_andn1_saveexec_b32",
    "s_andn1_saveexec_b64",
    "s_andn1_wrexec_b32",
    "s_andn1_wrexec_b64",
    "s_andn2_b32",
    "s_andn2_b64",
    "s_andn2_saveexec_b32",
    "s_andn2_saveexec_b64",
    "s_andn2_wrexec_b32",
    "s_andn2_wrexec_b64",
    "s_ashr_i32",
    "s_ashr_i64",
    "s_barrier",
    "s_bcnt0_i32_b32",
    "s_bcnt0_i32_b64",
    "s_bcnt1_i32_b32",
    "s_bcnt1_i32_b64",
    "s_bfe_i32",
    "s_bfe_i64",
    "s_bfe_u32",
    "s_bfe_u64",
    "s_bfm_b32",
    "s_bfm_b64",
    "s_bitcmp0_b32",
    "s_bitcmp0_b64",
    "s_bitcmp1_b32",
    "s_bitcmp1_b64",
    "s_bitreplicate_b64_b32",
    "s_bitset0_b32",
    "s_bitset0_b64",
    "s_bitset1_b32",
    "s_bitset1_b64",
    "s_branch",
    "s_brev_b32",
    "s_brev_b64",
    "s_buffer_load_dword",
    "s_buffer_load_dwordx16",
    "s_buffer_load_dwordx2",
    "s_buffer_load_dwordx4",
    "s_buffer_load_dwordx8",
    "s_call_b64",
    "s_cbranch_cdbgsys",
    "s_cbranch_cdbgsys_and_user",
    "s_cbranch_cdbgsys_or_user",
    "s_cbranch_cdbguser",
    "s_cbranch_execnz",
    "s_cbranch_execz",
    "s_cbranch_scc0",
    "s_cbranch_scc1",
    "s_cbranch_vccnz",
    "s_cbranch_vccz",
    "s_clause",
    "s_cmov_b32",
    "s_cmov_b64",
    "s_cmovk_i32",
    "s_cmp_eq_i32",
    "s_cmp_eq_u32",
    "s_cmp_eq_u64",
    "s_cmp_ge_i32",
    "s_cmp_ge_u32",
    "s_cmp_gt_i32",
    "s_cmp_gt_u32",
    "s_cmp_le_i32",
    "s_cmp_le_u32",
    "s_cmp_lg_i32",
    "s_cmp_lg_u32",
    "s_cmp_lg_u64",
    "s_cmp_lt_i32",
    "s_cmp_lt_u32",
    "s_cmpk_eq_i32",
    "s_cmpk_eq_u32",
    "s_cmpk_ge_i32",
    "s_cmpk_ge_u32",
    "s_cmpk_gt_i32",
    "s_cmpk_gt_u32",
    "s_cmpk_le_i32",
    "s_cmpk_le_u32",
    "s_cmpk_lg_i32",
    "s_cmpk_lg_u32",
    "s_cmpk_lt_i32",
    "s_cmpk_lt_u32",
    "s_code_end",
    "s_cselect_b32",
    "s_cselect_b64",
    "s_dcache_inv",
    "s_decperflevel",
    "s_denorm_mode",
    "s_endpgm",
    "s_endpgm_ordered_ps_done",
    "s_endpgm_saved",
    "s_ff0_i32_b32",
    "s_ff0_i32_b64",
    "s_ff1_i32_b32",
    "s_ff1_i32_b64",
    "s_flbit_i32",
    "s_flbit_i32_b32",
    "s_flbit_i32_b64",
    "s_flbit_i32_i64",
    "s_getpc_b64",
    "s_getreg_b32",
    "s_gl1_inv",
    "s_icache_inv",
    "s_incperflevel",
    "s_inst_prefetch",
    "s_load_dword",
    "s_load_dwordx16",
    "s_load_dwordx2",
    "s_load_dwordx4",
    "s_load_dwordx8",
    "s_lshl1_add_u32",
    "s_lshl2_add_u32",
    "s_lshl3_add_u32",
    "s_lshl4_add_u32",
    "s_lshl_b32",
    "s_lshl_b64",
    "s_lshr_b32",
    "s_lshr_b64",
    "s_max_i32",
    "s_max_u32",
    "s_memrealtime",
    "s_memtime",
    "s_min_i32",
    "s_min_u32",
    "s_mov_b32",
    "s_mov_b64",
    "s_movk_i32",
    "s_movreld_b32",
    "s_movreld_b64",
    "s_movrels_b32",
    "s_movrels_b64",
    "s_movrelsd_2_b32",
    "s_mul_hi_i32",
    "s_mul_hi_u32",
    "s_mul_i32",
    "s_mulk_i32",
    "s_nand_b32",
    "s_nand_b64",
    "s_nand_saveexec_b32",
    "s_nand_saveexec_b64",
    "s_nop",
    "s_nor_b32",
    "s_nor_b64",
    "s_nor_saveexec_b32",
    "s_nor_saveexec_b64",
    "s_not_b32",
    "s_not_b64",
    "s_or_b32",
    "s_or_b64",
    "s_or_saveexec_b32",
    "s_or_saveexec_b64",
    "s_orn1_saveexec_b32",
    "s_orn1_saveexec_b64",
    "s_orn2_b32",
    "s_orn2_b64",
    "s_orn2_saveexec_b32",
    "s_orn2_saveexec_b64",
    "s_pack_hh_b32_b16",
    "s_pack_lh_b32_b16",
    "s_pack_ll_b32_b16",
    "s_quadmask_b32",
    "s_quadmask_b64",
    "s_rfe_b64",
    "s_round_mode",
    "s_sendmsg",
    "s_sendmsghalt",
    "s_sethalt",
    "s_setkill",
    "s_setpc_b64",
    "s_setprio",
    "s_setreg_b32",
    "s_setreg_imm32_b32",
    "s_sext_i32_i16",
    "s_sext_i32_i8",
    "s_sleep",
    "s_sub_i32",
    "s_sub_u32",
    "s_subb_u32",
    "s_subvector_loop_begin",
    "s_subvector_loop_end",
    "s_swappc_b64",
    "s_trap",
    "s_ttracedata",
    "s_ttracedata_imm",
    "s_version",
    "s_waitcnt",
    "s_waitcnt_depctr",
    "s_waitcnt_expcnt",
    "s_waitcnt_lgkmcnt",
    "s_waitcnt_vmcnt",
    "s_waitcnt_vscnt",
    "s_wakeup",
    "s_wqm_b32",
    "s_wqm_b64",
    "s_xnor_b32",
    "s_xnor_b64",
    "s_xnor_saveexec_b32",
    "s_xnor_saveexec_b64",
    "s_xor_b32",
    "s_xor_b64",
    "s_xor_saveexec_b32",
    "s_xor_saveexec_b64",
    "scratch_load_dword",
    "scratch_load_dwordx2",
    "scratch_load_dwordx3",
    "scratch_load_dwordx4",
    "scratch_load_sbyte",
    "scratch_load_sbyte_d16",
    "scratch_load_sbyte_d16_hi",
    "scratch_load_short_d16",
    "scratch_load_short_d16_hi",
    "scratch_load_sshort",
    "scratch_load_ubyte",
    "scratch_load_ubyte_d16",
    "scratch_load_ubyte_d16_hi",
    "scratch_load_ushort",
    "scratch_store_byte",
    "scratch_store_byte_d16_hi",
    "scratch_store_dword",
    "scratch_store_dwordx2",
    "scratch_store_dwordx3",
    "scratch_store_dwordx4",
    "scratch_store_short",
    "scratch_store_short_d16_hi",
    "tbuffer_load_format_d16_x",
    "tbuffer_load_format_d16_xy",
    "tbuffer_load_format_d16_xyz",
    "tbuffer_load_format_d16_xyzw",
    "tbuffer_load_format_x",
    "tbuffer_load_format_xy",
    "tbuffer_load_format_xyz",
    "tbuffer_load_format_xyzw",
    "tbuffer_store_format_d16_x",
    "tbuffer_store_format_d16_xy",
    "tbuffer_store_format_d16_xyz",
    "tbuffer_store_format_d16_xyzw",
    "tbuffer_store_format_x",
    "tbuffer_store_format_xy",
    "tbuffer_store_format_xyz",
    "tbuffer_store_format_xyzw",
    "v_add3_u32",
    "v_add_co_ci_u32",
    "v_add_co_u32",
    "v_add_f16",
    "v_add_f32",
    "v_add_f64",
    "v_add_lshl_u32",
    "v_add_nc_i16",
    "v_add_nc_i32",
    "v_add_nc_u16",
    "v_add_nc_u32",
    "v_alignbit_b32",
    "v_alignbyte_b32",
    "v_and_b32",
    "v_and_or_b32",
    "v_ashrrev_i16",
    "v_ashrrev_i32",
    "v_ashrrev_i64",
    "v_bcnt_u32_b32",
    "v_bfe_i32",
    "v_bfe_u32",
    "v_bfi_b32",
    "v_bfm_b32",
    "v_bfrev_b32",
    "v_ceil_f16",
    "v_ceil_f32",
    "v_ceil_f64",
    "v_clrexcp",
    "v_cmp_class_f16",
    "v_cmp_class_f32",
    "v_cmp_class_f64",
    "v_cmp_eq_f16",
    "v_cmp_eq_f32",
    "v_cmp_eq_f64",
    "v_cmp_eq_i16",
    "v_cmp_eq_i32",
    "v_cmp_eq_i64",
    "v_cmp_eq_u16",
    "v_cmp_eq_u32",
    "v_cmp_eq_u64",
    "v_cmp_f_f16",
    "v_cmp_f_f32",
    "v_cmp_f_f64",
    "v_cmp_f_i32",
    "v_cmp_f_i64",
    "v_cmp_f_u32",
    "v_cmp_f_u64",
    "v_cmp_ge_f16",
    "v_cmp_ge_f32",
    "v_cmp_ge_f64",
    "v_cmp_ge_i16",
    "v_cmp_ge_i32",
    "v_cmp_ge_i64",
    "v_cmp_ge_u16",
    "v_cmp_ge_u32",
    "v_cmp_ge_u64",
    "v_cmp_gt_f16",
    "v_cmp_gt_f32",
    "v_cmp_gt_f64",
    "v_cmp_gt_i16",
    "v_cmp_gt_i32",
    "v_cmp_gt_i64",
    "v_cmp_gt_u16",
    "v_cmp_gt_u32",
    "v_cmp_gt_u64",
    "v_cmp_le_f16",
    "v_cmp_le_f32",
    "v_cmp_le_f64",
    "v_cmp_le_i16",
    "v_cmp_le_i32",
    "v_cmp_le_i64",
    "v_cmp_le_u16",
    "v_cmp_le_u32",
    "v_cmp_le_u64",
    "v_cmp_lg_f16",
    "v_cmp_lg_f32",
    "v_cmp_lg_f64",
    "v_cmp_lt_f16",
    "v_cmp_lt_f32",
    "v_cmp_lt_f64",
    "v_cmp_lt_i16",
    "v_cmp_lt_i32",
    "v_cmp_lt_i64",
    "v_cmp_lt_u16",
    "v_cmp_lt_u32",
    "v_cmp_lt_u64",
    "v_cmp_ne_i16",
    "v_cmp_ne_i32",
    "v_cmp_ne_i64",
    "v_cmp_ne_u16",
    "v_cmp_ne_u32",
    "v_cmp_ne_u64",
    "v_cmp_neq_f16",
    "v_cmp_neq_f32",
    "v_cmp_neq_f64",
    "v_cmp_nge_f16",
    "v_cmp_nge_f32",
    "v_cmp_nge_f64",
    "v_cmp_ngt_f16",
    "v_cmp_ngt_f32",
    "v_cmp_ngt_f64",
    "v_cmp_nle_f16",
    "v_cmp_nle_f32",
    "v_cmp_nle_f64",
    "v_cmp_nlg_f16",
    "v_cmp_nlg_f32",
    "v_cmp_nlg_f64",
    "v_cmp_nlt_f16",
    "v_cmp_nlt_f32",
    "v_cmp_nlt_f64",
    "v_cmp_o_f16",
    "v_cmp_o_f32",
    "v_cmp_o_f64",
    "v_cmp_t_i32",
    "v_cmp_t_i64",
    "v_cmp_t_u32",
    "v_cmp_t_u64",
    "v_cmp_tru_f16",
    "v_cmp_tru_f32",
    "v_cmp_tru_f64",
    "v_cmp_u_f16",
    "v_cmp_u_f32",
    "v_cmp_u_f64",
    "v_cmpx_class_f16",
    "v_cmpx_class_f32",
    "v_cmpx_class_f64",
    "v_cmpx_eq_f16",
    "v_cmpx_eq_f32",
    "v_cmpx_eq_f64",
    "v_cmpx_eq_i16",
    "v_cmpx_eq_i32",
    "v_cmpx_eq_i64",
    "v_cmpx_eq_u16",
    "v_cmpx_eq_u32",
    "v_cmpx_eq_u64",
    "v_cmpx_f_f16",
    "v_cmpx_f_f32",
    "v_cmpx_f_f64",
    "v_cmpx_f_i32",
    "v_cmpx_f_i64",
    "v_cmpx_f_u32",
    "v_cmpx_f_u64",
    "v_cmpx_ge_f16",
    "v_cmpx_ge_f32",
    "v_cmpx_ge_f64",
    "v_cmpx_ge_i16",
    "v_cmpx_ge_i32",
    "v_cmpx_ge_i64",
    "v_cmpx_ge_u16",
    "v_cmpx_ge_u32",
    "v_cmpx_ge_u64",
    "v_cmpx_gt_f16",
    "v_cmpx_gt_f32",
    "v_cmpx_gt_f64",
    "v_cmpx_gt_i16",
    "v_cmpx_gt_i32",
    "v_cmpx_gt_i64",
    "v_cmpx_gt_u16",
    "v_cmpx_gt_u32",
    "v_cmpx_gt_u64",
    "v_cmpx_le_f16",
    "v_cmpx_le_f32",
    "v_cmpx_le_f64",
    "v_cmpx_le_i16",
    "v_cmpx_le_i32",
    "v_cmpx_le_i64",
    "v_cmpx_le_u16",
    "v_cmpx_le_u32",
    "v_cmpx_le_u64",
    "v_cmpx_lg_f16",
    "v_cmpx_lg_f32",
    "v_cmpx_lg_f64",
    "v_cmpx_lt_f16",
    "v_cmpx_lt_f32",
    "v_cmpx_lt_f64",
    "v_cmpx_lt_i16",
    "v_cmpx_lt_i32",
    "v_cmpx_lt_i64",
    "v_cmpx_lt_u16",
    "v_cmpx_lt_u32",
    "v_cmpx_lt_u64",
    "v_cmpx_ne_i16",
    "v_cmpx_ne_i32",
    "v_cmpx_ne_i64",
    "v_cmpx_ne_u16",
    "v_cmpx_ne_u32",
    "v_cmpx_ne_u64",
    "v_cmpx_neq_f16",
    "v_cmpx_neq_f32",
    "v_cmpx_neq_f64",
    "v_cmpx_nge_f16",
    "v_cmpx_nge_f32",
    "v_cmpx_nge_f64",
    "v_cmpx_ngt_f16",
    "v_cmpx_ngt_f32",
    "v_cmpx_ngt_f64",
    "v_cmpx_nle_f16",
    "v_cmpx_nle_f32",
    "v_cmpx_nle_f64",
    "v_cmpx_nlg_f16",
    "v_cmpx_nlg_f32",
    "v_cmpx_nlg_f64",
    "v_cmpx_nlt_f16",
    "v_cmpx_nlt_f32",
    "v_cmpx_nlt_f64",
    "v_cmpx_o_f16",
    "v_cmpx_o_f32",
    "v_cmpx_o_f64",
    "v_cmpx_t_i32",
    "v_cmpx_t_i64",
    "v_cmpx_t_u32",
    "v_cmpx_t_u64",
    "v_cmpx_tru_f16",
    "v_cmpx_tru_f32",
    "v_cmpx_tru_f64",
    "v_cmpx_u_f16",
    "v_cmpx_u_f32",
    "v_cmpx_u_f64",
    "v_cndmask_b32",
    "v_cos_f16",
    "v_cos_f32",
    "v_cubeid_f32",
    "v_cubema_f32",
    "v_cubesc_f32",
    "v_cubetc_f32",
    "v_cvt_f16_f32",
    "v_cvt_f16_i16",
    "v_cvt_f16_u16",
    "v_cvt_f32_f16",
    "v_cvt_f32_f64",
    "v_cvt_f32_i32",
    "v_cvt_f32_u32",
    "v_cvt_f32_ubyte0",
    "v_cvt_f32_ubyte1",
    "v_cvt_f32_ubyte2",
    "v_cvt_f32_ubyte3",
    "v_cvt_f64_f32",
    "v_cvt_f64_i32",
    "v_cvt_f64_u32",
    "v_cvt_flr_i32_f32",
    "v_cvt_i16_f16",
    "v_cvt_i32_f32",
    "v_cvt_i32_f64",
    "v_cvt_norm_i16_f16",
    "v_cvt_norm_u16_f16",
    "v_cvt_off_f32_i4",
    "v_cvt_pk_i16_i32",
    "v_cvt_pk_u16_u32",
    "v_cvt_pk_u8_f32",
    "v_cvt_pknorm_i16_f16",
    "v_cvt_pknorm_i16_f32",
    "v_cvt_pknorm_u16_f16",
    "v_cvt_pknorm_u16_f32",
    "v_cvt_pkrtz_f16_f32",
    "v_cvt_rpi_i32_f32",
    "v_cvt_u16_f16",
    "v_cvt_u32_f32",
    "v_cvt_u32_f64",
    "v_div_fixup_f16",
    "v_div_fixup_f32",
    "v_div_fixup_f64",
    "v_div_fmas_f32",
    "v_div_fmas_f64",
    "v_div_scale_f32",
    "v_div_scale_f64",
    "v_dot2_f32_f16",
    "v_dot2_i32_i16",
    "v_dot2_u32_u16",
    "v_dot2c_f32_f16",
    "v_dot4_i32_i8",
    "v_dot4_u32_u8",
    "v_dot4c_i32_i8",
    "v_dot8_i32_i4",
    "v_dot8_u32_u4",
    "v_exp_f16",
    "v_exp_f32",
    "v_ffbh_i32",
    "v_ffbh_u32",
    "v_ffbl_b32",
    "v_floor_f16",
    "v_floor_f32",
    "v_floor_f64",
    "v_fma_f16",
    "v_fma_f32",
    "v_fma_f64",
    "v_fma_legacy_f32",
    "v_fma_mix_f32",
    "v_fma_mixhi_f16",
    "v_fma_mixlo_f16",
    "v_fmaak_f16",
    "v_fmaak_f32",
    "v_fmac_f16",
    "v_fmac_f32",
    "v_fmac_legacy_f32",
    "v_fmamk_f16",
    "v_fmamk_f32",
    "v_fract_f16",
    "v_fract_f32",
    "v_fract_f64",
    "v_frexp_exp_i16_f16",
    "v_frexp_exp_i32_f32",
    "v_frexp_exp_i32_f64",
    "v_frexp_mant_f16",
    "v_frexp_mant_f32",
    "v_frexp_mant_f64",
    "v_interp_mov_f32",
    "v_interp_p1_f32",
    "v_interp_p1ll_f16",
    "v_interp_p1lv_f16",
    "v_interp_p2_f16",
    "v_interp_p2_f32",
    "v_ldexp_f16",
    "v_ldexp_f32",
    "v_ldexp_f64",
    "v_lerp_u8",
    "v_log_f16",
    "v_log_f32",
    "v_lshl_add_u32",
    "v_lshl_or_b32",
    "v_lshlrev_b16",
    "v_lshlrev_b32",
    "v_lshlrev_b64",
    "v_lshrrev_b16",
    "v_lshrrev_b32",
    "v_lshrrev_b64",
    "v_mad_i16",
    "v_mad_i32_i16",
    "v_mad_i32_i24",
    "v_mad_i64_i32",
    "v_mad_u16",
    "v_mad_u32_u16",
    "v_mad_u32_u24",
    "v_mad_u64_u32",
    "v_max3_f16",
    "v_max3_f32",
    "v_max3_i16",
    "v_max3_i32",
    "v_max3_u16",
    "v_max3_u32",
    "v_max_f16",
    "v_max_f32",
    "v_max_f64",
    "v_max_i16",
    "v_max_i32",
    "v_max_u16",
    "v_max_u32",
    "v_mbcnt_hi_u32_b32",
    "v_mbcnt_lo_u32_b32",
    "v_med3_f16",
    "v_med3_f32",
    "v_med3_i16",
    "v_med3_i32",
    "v_med3_u16",
    "v_med3_u32",
    "v_min3_f16",
    "v_min3_f32",
    "v_min3_i16",
    "v_min3_i32",
    "v_min3_u16",
    "v_min3_u32",
    "v_min_f16",
    "v_min_f32",
    "v_min_f64",
    "v_min_i16",
    "v_min_i32",
    "v_min_u16",
    "v_min_u32",
    "v_mov_b32",
    "v_movreld_b32",
    "v_movrels_b32",
    "v_movrelsd_2_b32",
    "v_movrelsd_b32",
    "v_mqsad_pk_u16_u8",
    "v_mqsad_u32_u8",
    "v_msad_u8",
    "v_mul_f16",
    "v_mul_f32",
    "v_mul_f64",
    "v_mul_hi_i32",
    "v_mul_hi_i32_i24",
    "v_mul_hi_u32",
    "v_mul_hi_u32_u24",
    "v_mul_i32_i24",
    "v_mul_legacy_f32",
    "v_mul_lo_u16",
    "v_mul_lo_u32",
    "v_mul_u32_u24",
    "v_mullit_f32",
    "v_nop",
    "v_not_b32",
    "v_or3_b32",
    "v_or_b32",
    "v_pack_b32_f16",
    "v_perm_b32",
    "v_permlane16_b32",
    "v_permlanex16_b32",
    "v_pipeflush",
    "v_pk_add_f16",
    "v_pk_add_i16",
    "v_pk_add_u16",
    "v_pk_ashrrev_i16",
    "v_pk_fma_f16",
    "v_pk_fmac_f16",
    "v_pk_lshlrev_b16",
    "v_pk_lshrrev_b16",
    "v_pk_mad_i16",
    "v_pk_mad_u16",
    "v_pk_max_f16",
    "v_pk_max_i16",
    "v_pk_max_u16",
    "v_pk_min_f16",
    "v_pk_min_i16",
    "v_pk_min_u16",
    "v_pk_mul_f16",
    "v_pk_mul_lo_u16",
    "v_pk_sub_i16",
    "v_pk_sub_u16",
    "v_qsad_pk_u16_u8",
    "v_rcp_f16",
    "v_rcp_f32",
    "v_rcp_f64",
    "v_rcp_iflag_f32",
    "v_readfirstlane_b32",
    "v_readlane_b32",
    "v_rndne_f16",
    "v_rndne_f32",
    "v_rndne_f64",
    "v_rsq_f16",
    "v_rsq_f32",
    "v_rsq_f64",
    "v_sad_hi_u8",
    "v_sad_u16",
    "v_sad_u32",
    "v_sad_u8",
    "v_sat_pk_u8_i16",
    "v_sin_f16",
    "v_sin_f32",
    "v_sqrt_f16",
    "v_sqrt_f32",
    "v_sqrt_f64",
    "v_sub_co_ci_u32",
    "v_sub_co_u32",
    "v_sub_f16",
    "v_sub_f32",
    "v_sub_nc_i16",
    "v_sub_nc_i32",
    "v_sub_nc_u16",
    "v_sub_nc_u32",
    "v_subrev_co_ci_u32",
    "v_subrev_co_u32",
    "v_subrev_f16",
    "v_subrev_f32",
    "v_subrev_nc_u32",
    "v_swap_b32",
    "v_swaprel_b32",
    "v_trig_preop_f64",
    "v_trunc_f16",
    "v_trunc_f32",
    "v_trunc_f64",
    "v_writelane_b32",
    "v_xad_u32",
    "v_xnor_b32",
    "v_xor3_b32",
    "v_xor_b32",
];
