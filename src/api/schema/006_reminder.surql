DEFINE TABLE reminder SCHEMAFULL;

DEFINE FIELD user ON TABLE reminder TYPE string ASSERT $value != NONE;
DEFINE FIELD medication ON TABLE reminder TYPE record(medication) ASSERT $value != NONE;
DEFINE FIELD start ON TABLE reminder TYPE datetime ASSERT $value != NONE;
DEFINE FIELD end ON TABLE reminder TYPE datetime ASSERT $value != NONE;
DEFINE FIELD days ON TABLE reminder TYPE string ASSERT $value != NONE AND $value = /[01]{7}/;
DEFINE FIELD times ON TABLE reminder TYPE array;
DEFINE FIELD times.* ON TABLE reminder TYPE string;
DEFINE FIELD active ON TABLE reminder TYPE bool VALUE $value ?? true;  -- Defaulting the value to true if not provide
DEFINE FIELD created ON reminder VALUE $before OR time::now();
DEFINE FIELD updated ON reminder VALUE time::now();

//  Indexes
DEFINE INDEX reminder_index ON TABLE reminder FIELDS user, medication, start, active UNIQUE;
