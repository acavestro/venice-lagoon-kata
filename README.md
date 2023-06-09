# The Venice Lagoon Kata

The Venice Lagoon is really influenced by the tides effects. High tides may increase the water level over the level of the streets (a street in Venice is called "calle"), causing issues to houses, pedestrians and services.

Let's build a notification system that sends alert to subscribers in case of dangerous high tides. Suppose to have a CSV file with some weather forecast data for tomorrow. Levels are expressed in cm.

```
date, time, level
2023-06-01, 04:15, -15
2023-06-01, 10:30, 55
2023-06-01, 15:20, 15
2023-06-01, 21:40, 90
```

Every day, at midnight, the system reads the forecast and provides three levels of notification:

* **Yellow warning**: the water level is above of the lowest place in Venice, which is 80cm high
* **Orange warning**: the water level is above Rialto area height, which is 105cm high. It means that 5% of the city is flooded.
* **Red warning**: the water level is above Train Station height (Santa Lucia), which is 135cm high. Almost 50% of the city is flooded

Notifications can be sent via sms or emails. Here is the message template:

```
Hello {name},
today the high tide is forecast to be at {color} warning level. The highest peak will be at {time}.
```

Regarding the subscribed users, they are defined in another CSV file with these columns:

```
name, surname, email, phone
Foo, Bar, foo@bar.com, 3331234567
John, Doe, john@doe.org, 3330987654
```
