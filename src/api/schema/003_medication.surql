DEFINE TABLE medication SCHEMAFULL;

DEFINE FIELD user ON TABLE medication TYPE record(user) ASSERT $value != NONE;
DEFINE FIELD name ON TABLE medication TYPE string ASSERT $value != NONE;
DEFINE FIELD active ON TABLE medication TYPE bool VALUE $value ?? true;  -- Defaulting the value to true if not provide
DEFINE FIELD created ON medication VALUE $before OR time::now();
DEFINE FIELD updated ON medication VALUE time::now();

//  Indexes
DEFINE INDEX medication_index ON medication FIELDS name UNIQUE;

// Events
//TODO: Verify structure once Beta 10 is released
DEFINE EVENT name ON medication WHEN $before.name != $after.name THEN
(CREATE event SET medication = $value.id, time = time::now(), value = $after.name,
event = 'name_changed', field = medication:name)

// Functions
DEFINE FUNCTION fn::list_user_medications($user: string) {LET $results =
(SELECT created, id, name, updated, active, user FROM medication WHERE user = type::thing('user', $user)
ORDER BY created);
RETURN $results;};

DEFINE FUNCTION fn::list_user_medications_by_status($active: bool, $user: string) {LET $results =
(SELECT created, id, name, updated, active, user FROM medication WHERE user = type::thing('user', $user)
AND active = $active ORDER BY created);
RETURN $results;};
