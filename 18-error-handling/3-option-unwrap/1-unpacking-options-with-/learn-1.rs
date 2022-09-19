struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {

    // Gets the area code of the phone number of the person's job, if it exists.
    fn work_phone_area_code(&self) -> Option<u8> {
        // This would need many nested `match` statements without the `?` operator.
        // It would take a lot more code - try writing it yourself and see which
        // is easier.
        match self.job {
            Some(job) => match job.phone_number {
                Some(phone_number) => phone_number.area_code,
                None => None
            },
            None => None,
        }
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        })
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}
