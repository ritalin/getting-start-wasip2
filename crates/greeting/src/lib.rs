mod greeting;
use greeting::exports::anonymous::greeting::say;

struct Component;

impl say::Guest for Component {
    // _rt::Stringと出るけど、単なるstd::string::Stringのエイリアス
    fn hello() -> String {
        "Hello World !!".into()
    }
}

greeting::export!(Component with_types_in greeting);