pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            super::super::deliver_order();
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

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
