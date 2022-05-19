use super::CustomError;

pub trait ToCustomErrorTrait<T, S>{
    fn to_custom_error(&self, message: &str) -> Result<T, CustomError>;
}