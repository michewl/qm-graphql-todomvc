@todo
Feature: Remove todos
  As a user
  I want to be able to delete todos

  Scenario Template: If todos get deleted, they are removed from the database
    Given a todo with title "first" exists
    Given a todo with title "second" exists
    Given a todo with title "third" exists
    When removeTodos is sent with ids for "<titles>"
    Then the todos with titles "<titles>" are not in the database
    And the todo with title "first" is in the database
    And the response has no errors
    And the response data is integer value <result_value>

    Examples:
      | titles        | result_value |
      | second        |            1 |
      | second, third |            2 |
