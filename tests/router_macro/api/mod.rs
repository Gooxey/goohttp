use goohttp::router;

router! {
    api {
        say_hello, get, ":caller";
        say_hello_caller_sender, get, ":caller", ":sender"
    }
}