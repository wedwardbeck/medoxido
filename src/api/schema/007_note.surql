DEFINE TABLE note SCHEMAFULL;

DEFINE FIELD user ON TABLE note TYPE record(user) ASSERT $value != NONE;
DEFINE FIELD note_table ON TABLE note TYPE string ASSERT $value != NONE;
DEFINE FIELD note_thing ON TABLE note TYPE string ASSERT $value != NONE;
DEFINE FIELD content ON TABLE note TYPE string ASSERT $value != NONE;
DEFINE FIELD created ON note VALUE $before OR time::now();
DEFINE FIELD updated ON note VALUE time::now();
