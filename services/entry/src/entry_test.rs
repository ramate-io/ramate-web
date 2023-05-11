#[cfg(test)]
mod entry_tests {

    use std::vec;

    use crate::entry::{Project};
    use prost::Message;

    #[test]
    fn exploration() {
        let proj = Project {
            id : "hello".to_string(),
            description : "hello".to_string(),
            milestones : vec![]
        };

        println!("{:?}", proj.encode_to_vec());

    }

}