@tag
Feature: Remove tags
  As a user
  I want to be able to delete tags

  Scenario Template: If tags get deleted, they are removed from the database
    Given a tag with name "first" exists
    Given a tag with name "second" exists
    Given a tag with name "third" exists
    When removeTags is sent with ids for "<names>"
    Then the tags with names "<names>" are not in the database
    And the tag with name "first" is in the database
    And the response has no errors
    And the response data is integer value <result_value>

    Examples:
      | names         | result_value |
      | second        |            1 |
      | second, third |            2 |
