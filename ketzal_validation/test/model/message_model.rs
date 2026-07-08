use ketzal_validation::KetzalBasicM;

#[derive(KetzalBasicM)]
pub struct MessageModel {
    #[rule("min:3")]
    #[message(min = "Custom :attribute min message")]
    pub name: String,
    #[rule("min:5|max:20")]
    #[message(
        min = "The :attribute needs at least :min chars",
        max = "The :attribute must not exceed :max chars"
    )]
    pub email: Option<String>,
}
