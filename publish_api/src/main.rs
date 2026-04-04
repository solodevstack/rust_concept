// use publish_api::kinds::PrimaryColor;
// use publish_api::utils::mix;
use publish_api::PrimaryColor;
use publish_api::mix;


fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}