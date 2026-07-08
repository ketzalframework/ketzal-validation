use ketzal_validation::KetzalBasicM;

#[derive(KetzalBasicM)]
pub struct InModel {
    #[rule("in:admin,user,guest")]
    pub role: String,
    #[rule("not_in:banned,deleted")]
    pub status: Option<String>,
}
