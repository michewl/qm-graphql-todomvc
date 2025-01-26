@tag
Feature: Update tag
  As a user
  I want to be able to update a tag

  Scenario: If a tag is updated, the new values are stored in the database
    Given a tag with name "test" exists
    When updateTag is sent with body
      """
      {"name": "updated-test", "id": "replaced-by-step-function"}
      """
    Then the response has no errors
    And the given tag has field name with string value "updated-test"
    And the response data JSON node "$.modified" should have a value

  Rule: The name must be unique

    Scenario: If a tag is updated to a duplicate name, it is rejected
      Given a tag with name "duplicate" exists
      And a tag with name "test" exists
      When updateTag is sent with body
        """
          {"name": "duplicate", "id": "replaced-by-step-function"}
        """
      Then the response should have errors
      And a response error with message containing "dup key: { name: \"duplicate\" }" exists
