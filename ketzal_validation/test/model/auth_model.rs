use ketzal_validation::KetzalBasicM;

#[derive(KetzalBasicM)]
pub struct RegisterRequest {
    #[rule("min:8|confirmed")]
    pub password: String,
    pub password_confirmation: String,
}
