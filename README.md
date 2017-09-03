# date-iterator

This crate provides two concepts on top of (`chrono`)[https://github.com/chronotope/chrono]:

* `CalendarDuration` that is able to add months and years (which have varying lengths and thus cannot be represented as seconds)
* `date_iterators` that can be used to iterate over date ranges:
    * `OpenEndedDateIterator` which never stops
    * `ClosedDateIterator` which stops after a given date
