@setup
Feature: Tests run with clean setup
  As a user
  I want that tests run on a clean setup

  Rule: The test database must not contain data

    Scenario Template: Every scenario runs on an empty database
      When test number <n> is ran
      Then all database collections are empty

      Examples:
        | n |
        | 1 |
        | 2 |
