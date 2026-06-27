use ketzal_validation::KetzalBasicM;

#[derive(KetzalBasicM)]
pub struct User {
    #[rule("min:3|max:10")]
    pub name: String,
    #[rule("min:2|max:20")]
    pub midelnames: Option<String>,
    #[rule("min:5|max:50")]
    pub email: String,
}
