
DEFINE TABLE dose SCHEMAFULL;

DEFINE FIELD user ON TABLE dose TYPE record(user) ASSERT $value != NONE;
DEFINE FIELD store ON TABLE dose TYPE record(store) ASSERT $value != NONE;
DEFINE FIELD quantity ON TABLE dose TYPE float ASSERT $value != NONE AND $value > 0;
DEFINE FIELD unit ON TABLE dose TYPE string ASSERT $value != NONE;
DEFINE FIELD created ON dose VALUE $before OR time::now();
DEFINE FIELD updated ON dose VALUE time::now();
