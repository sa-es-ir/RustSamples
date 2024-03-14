pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist called");
        }

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
