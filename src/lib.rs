#[cfg(test)]
mod test {
    use chrono::{Date, DateTime, NaiveDate, Utc};
    use rstest::rstest;

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
        // * **Yellow warning**: the water level is above of the lowest place in Venice, which is 80cm high
        // * **Orange warning**: the water level is above Rialto area height, which is 105cm high. It means that 5% of the city is flooded.
        // * **Red warning**: the water level is above Train Station height (Santa Lucia), which is 135cm high. Almost 50% of the city is flooded

        fn level_as_str(&self) -> &str {
            match self.level {
                level if level < 80 => "green",
                level if level >= 80 && level < 105 => "yellow",
                level if level >= 105 && level < 135 => "orange",
                _ => "red",
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
        fn from_text(text: impl ToString) -> Self {
            Self {
                text: text.to_string(),
            }
        }
        fn new(measurement: &Measurement, subscriber: &Subscriber) -> Option<Self> {
            Some(Self::from_text(format!(
                r#"Hello {},
            today the high tide is forecast to be at {} warning level. The highest peak will be at {}."#,
                subscriber.name,
                measurement.level_as_str(),
                measurement.time,
            )))
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
            let measurements = self
                .measurements
                .get(NaiveDate::from_ymd_opt(2023, 6, 23).expect("invalid date"))?;
            let subscribers = self.subscribers.get()?;
            let subscriber = subscribers.first().unwrap().clone();
            let Some(measurement) = measurements.first() else {
                return Ok(());
            };
            let notification = Notification::new(measurement, &subscriber).unwrap();
            self.sender.send(subscriber, notification)
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
        fn get(&self, date: NaiveDate) -> Result<Vec<Measurement>, String>;
    }

    #[rstest]
    #[case(Measurement::new("2023-06-1", "04:15", 80), Subscriber::new("Foo Bar", "foo@bar.com", "3331234567"), Notification::from_text(
            r#"Hello Foo Bar,
            today the high tide is forecast to be at yellow warning level. The highest peak will be at 04:15."#,
        ))]
    #[case(Measurement::new("2023-06-1", "10:10", 110), Subscriber::new("Tizio Caso", "foo@bar.com", "3331234567"), Notification::from_text(
            r#"Hello Tizio Caso,
            today the high tide is forecast to be at orange warning level. The highest peak will be at 10:10."#,
        ))]
    fn subscribers_receive_notification_given_a_measurement_for_today(
        #[case] measurement: Measurement,
        #[case] subscriber: Subscriber,
        #[case] expected_notification: Notification,
    ) {
        let expected_subscriber = subscriber.clone();

        let mut measurements = MockMeasurements::new();
        measurements
            .expect_get()
            .times(1)
            .return_once(move |_| Ok(vec![measurement]));

        let mut subscribers = MockSubscribers::new();
        subscribers
            .expect_get()
            .times(1)
            .return_once(move || Ok(vec![subscriber]));

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
        assert!(result.is_ok());
    }

    #[rstest]
    #[case::green(Measurement::new("2023-06-1", "04:15", 30), "green")]
    #[case::yellow(Measurement::new("2023-06-1", "04:15", 85), "yellow")]
    #[case::orange(Measurement::new("2023-06-1", "04:15", 120), "orange")]
    #[case::red(Measurement::new("2023-06-1", "04:15", 200), "red")]
    fn it_renders_the_level_as_a_warning_string(
        #[case] measurement: Measurement,
        #[case] expected_warning: &str,
    ) {
        assert_eq!(expected_warning, measurement.level_as_str());
    }

    #[rstest]
    fn subscribers_dont_receive_notification_when_no_measurement_for_today() {
        let measurement = Measurement::new("2023-06-1", "04:15", 80);
        let subscriber = Subscriber::new("Foo Bar", "foo@bar.com", "3331234567");
        let expected_subscriber = subscriber.clone();

        let mut measurements = MockMeasurements::new();
        measurements
            .expect_get()
            .times(1)
            .return_once(move |_| Ok(vec![]));

        let mut subscribers = MockSubscribers::new();
        subscribers
            .expect_get()
            .times(1)
            .return_once(move || Ok(vec![subscriber]));

        let mut sender = MockSender::new();
        sender.expect_send().times(0);

        let notifier = Notifier::new(
            Box::new(sender),
            Box::new(subscribers),
            Box::new(measurements),
        );

        let result = notifier.notify();
        assert!(result.is_ok());
    }
}
