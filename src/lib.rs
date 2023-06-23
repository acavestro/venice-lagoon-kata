#[cfg(test)]
mod test {
    use mockall::{automock, predicate::*};
    #[derive(Clone)]
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

    #[derive(Clone, Debug, PartialEq)]
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

    #[derive(Debug, PartialEq)]
    struct Notification {
        text: String,
    }

    impl Notification {
        fn new(text: impl ToString) -> Self {
            Self {
                text: text.to_string(),
            }
        }
        fn from_measurement(measurement: Measurement) -> Option<Self> {
            Some(Self::new(""))
        }
    }

    struct Notifier {
        sender: Box<dyn Sender>,
        subscribers: Box<dyn Subscribers>,
        measurements: Box<dyn Measurements>,
    }

    impl Notifier {
        fn new(
            sender: Box<dyn Sender>,
            subscribers: Box<dyn Subscribers>,
            measurements: Box<dyn Measurements>,
        ) -> Self {
            Self {
                sender,
                subscribers,
                measurements,
            }
        }

        fn notify(&self) -> Result<(), String> {
            let measurements = self.measurements.get()?;
            let subscribers = self.subscribers.get()?;
            self.sender.send(
                subscribers.first().unwrap().clone(),
                Notification::from_measurement(measurements.first().unwrap().clone()).unwrap(),
            )
        }
    }

    #[automock]
    trait Sender {
        fn send(&self, subscriber: Subscriber, notification: Notification) -> Result<(), String>;
    }

    #[automock]
    trait Subscribers {
        fn get(&self) -> Result<Vec<Subscriber>, String>;
    }

    #[automock]
    trait Measurements {
        fn get(&self) -> Result<Vec<Measurement>, String>;
    }

    #[test]
    fn subscribers_receive_notification_given_a_measurement_for_today() {
        let measurement = Measurement::new("2023-06-01", "04:15", -15);
        let subscriber = Subscriber::new("Foo Bar", "foo@bar.com", "3331234567");

        let expected_subscriber = subscriber.clone();

        let mut measurements = MockMeasurements::new();
        measurements
            .expect_get()
            .times(1)
            .return_once(move || Ok(vec![measurement]));

        let mut subscribers = MockSubscribers::new();
        subscribers
            .expect_get()
            .times(1)
            .return_once(move || Ok(vec![subscriber]));

        let expected_notification = Notification::new(
            r#"Hello Foo Bar,
            today the high tide is forecast to be at yellow warning level. The highest peak will be at {time}."#,
        );

        let mut sender = MockSender::new();
        sender
            .expect_send()
            .times(1)
            .with(eq(expected_subscriber), eq(expected_notification))
            .returning(|_, _| Ok(()));

        let notifier = Notifier::new(
            Box::new(sender),
            Box::new(subscribers),
            Box::new(measurements),
        );

        let result = notifier.notify();
    }

    //#[test]
    // fn subscribers_dont_receive_notification_when_no_measurement_for_today() {
    //     todo!()
    // }
}
