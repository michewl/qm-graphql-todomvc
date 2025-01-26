@todo
Feature: Update todo
  As a user
  I want to be able to update a todo

  Scenario: If a todo is updated, the new values are stored in the database
    Given a todo with title "test" exists
    When updateTodo is sent with body
      """
      {"completed": true, "title": "updated-test", "id": "replaced-by-step-function"}
      """
    Then the given todo has field title with string value "updated-test"
    And the given todo has field completed with boolean value true
    And the response has no errors
    And the response data JSON node "$.modified" should have a value
