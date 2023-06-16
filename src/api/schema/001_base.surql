// Define the Namespace and database
DEFINE NAMESPACE medirust;
DEFINE DATABASE medirust;

//TODO: Verify use of this section in a migration process, or is embedded creating the main db file?
// If embedded creates the main db file, then this section is not needed and we distribute with the
// binary.
USE NS medirust DB medirust;
// User Table
DEFINE TABLE user SCHEMAFULL;

DEFINE FIELD username ON TABLE user TYPE string ASSERT $value != NONE;
DEFINE FIELD email ON TABLE user TYPE string ASSERT $value != NONE && is::email($value);
DEFINE FIELD active ON TABLE user TYPE bool VALUE $value ?? true;  -- Defaulting the value to true if not provide
DEFINE FIELD created ON user VALUE $before OR time::now();
DEFINE FIELD updated ON user VALUE time::now();

//  User Table Indexes
DEFINE INDEX username_index ON user FIELDS username UNIQUE;

// Events
DEFINE EVENT email ON user WHEN $before.email != $after.email THEN
(CREATE event SET user = $value.id, time = time::now(), value = $after.email,
event = 'email_changed', field = user:email)"

//TODO: Add remaining core base objects here
