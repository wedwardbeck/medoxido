DEFINE TABLE reminder SCHEMAFULL;

DEFINE FIELD user ON TABLE reminder TYPE record(user) ASSERT $value != NONE;
DEFINE FIELD name ON TABLE reminder TYPE string ASSERT $value != NONE;
DEFINE FIELD active ON TABLE reminder TYPE bool VALUE $value ?? true;  -- Defaulting the value to true if not provide
DEFINE FIELD created ON reminder VALUE $before OR time::now();
DEFINE FIELD updated ON reminder VALUE time::now();

//  Indexes
DEFINE INDEX reminder_index ON medication FIELDS name UNIQUE;

// Events
//TODO: Verify structure once Beta 10 is released
DEFINE EVENT name ON medication WHEN $before.name != $after.name THEN
(CREATE event SET medication = $value.id, time = time::now(), value = $after.name,
event = 'name_changed', field = medication:name)
