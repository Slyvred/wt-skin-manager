// Contains the values of the filters
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SearchParams {
    pub country: Option<String>,
    pub v_type: Option<String>,
    pub class: Option<String>,
    pub vehicle: Option<String>,
}
