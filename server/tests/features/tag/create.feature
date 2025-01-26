@tag
Feature: Create tag
  As a user
  I want to be able to create tags

  Scenario: If a tag is created, it is stored in the database
    When createTag is sent with body
      """
      {"name": "test"}
      """
    Then the tag with name "test" is in the database
    And the response has no errors
    And the response data JSON node "$.name" should have the value "test"
    And the response data JSON node "$.created" should have a value

  Rule: The name must be unique

    Scenario: If a tag is created with a duplicate name, it is rejected
      Given a tag with name "test" exists
      When createTag is sent with body
        """
          {"name": "test"}
        """
      Then the response should have errors
      Then a response error with message containing "dup key: { name: \"test\" }" exists
