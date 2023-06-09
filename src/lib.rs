#[cfg(test)]
mod test {
    use mockall::predicate::*;
    struct Measurement {
        date: String,
        time: String,
        level: i32,
    }

    impl Measurement {
        fn new(date: impl ToString, time: impl ToString, level: i32) -> Self {
            Self {
                date: date.to_string(),
                time: time.to_string(),
                level,
            }
        }
    }

    struct Subscriber {
        name: String,
        email: String,
        phone_number: String,
    }

    impl Subscriber {
        fn new(name: impl ToString, email: impl ToString, phone_number: impl ToString) -> Self {
            Self {
                name: name.to_string(),
                email: email.to_string(),
                phone_number: phone_number.to_string(),
            }
        }
    }

    struct Notification;

    impl Notification {
        fn from_measurement(measurement: Measurement) -> Option<Self> {
            Some(Self)
        }
    }

    struct Notifier;

    impl Notifier {
        fn new() -> Self {
            Self
        }

        fn notify(&self, subscribers: &[Subscriber]) -> Result<(), String> {
            Ok(())
        }
    }

    #[automock]
    trait Sender {
        fn send(&self, subscriber: Subscriber, notification: Notification) -> Result<(), String>;
    }

    #[test]
    fn subscribers_receive_notification_given_a_measurement_for_today() {
        let measurement = Measurement::new("2023-06-01", "04:15", -15);
        let subscriber = Subscriber::new("Foo Bar", "foo@bar.com", "3331234567");
        let notification = Notification::from_measurement(measurement);

        assert!(notification.is_some());
        let notifier = Notifier::new();
        let subscribers = [subscriber];
        let result = notifier.notify(&subscribers);
        assert!(result.is_ok());
        let sender = MockSender::new();

        sender.expect_send().times(1).returning(|_| Ok(()));
    }

    //#[test]
    // fn subscribers_dont_receive_notification_when_no_measurement_for_today() {
    //     todo!()
    // }
}
