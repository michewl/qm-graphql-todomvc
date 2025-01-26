@todo
Feature: Create todo
  As a user
  I want to be able to create todos

  Scenario: If a todo is created, it is stored in the database
    When createTodo is sent with body
      """
      {"completed": false, "order": 1, "title": "test"}
      """
    Then the todo with title "test" is in the database
    And the response has no errors
    And the response data JSON node "$.title" should have the value "test"
    And the response data JSON node "$.created" should have a value
