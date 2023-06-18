DEFINE TABLE note SCHEMAFULL;

DEFINE FIELD user ON TABLE note TYPE record(user) ASSERT $value != NONE;
DEFINE FIELD note_table ON TABLE note TYPE string ASSERT $value != NONE;
DEFINE FIELD note_thing ON TABLE note TYPE string ASSERT $value != NONE;
DEFINE FIELD content ON TABLE note TYPE string ASSERT $value != NONE;
DEFINE FIELD created ON note VALUE $before OR time::now();
DEFINE FIELD updated ON note VALUE time::now();

// Functions
DEFINE FUNCTION fn::list_all_dose_notes(

) {let $results = (select id, content, created, note_table, note_thing, updated,
type::thing(note_table,note_thing) as dose_id,
type::thing(note_table,note_thing).quantity as dose_quantity,
type::thing(note_table,note_thing).unit as unit,
type::thing(note_table,note_thing).store.id as store_id,
type::thing(note_table,note_thing).store.quantity as store_start_quantity,
type::thing(note_table,note_thing).store.production_date as store_production_date,
type::thing(note_table,note_thing).created as dose_created,
type::thing(note_table,note_thing).updated as dose_updated,
type::thing(note_table,note_thing).store.medication as medication_id,
type::thing(note_table,note_thing).store.medication.name as medication_name from note where note_table = "dose" ORDER BY created);
RETURN $results;
};

DEFINE FUNCTION fn::list_notes_for_dose(
    $id: string
) {let $results = (select id, content, created,note_table, note_thing, updated,
type::thing(note_table,note_thing) as dose_id,
type::thing(note_table,note_thing).quantity as dose_quantity,
type::thing(note_table,note_thing).unit as unit,
type::thing(note_table,note_thing).store.id as store_id,
type::thing(note_table,note_thing).store.quantity as store_start_quantity,
type::thing(note_table,note_thing).store.production_date as store_production_date,
type::thing(note_table,note_thing).created as dose_created,
type::thing(note_table,note_thing).updated as dose_updated,
type::thing(note_table,note_thing).store.medication as medication_id,
type::thing(note_table,note_thing).store.medication.name as medication_name from note where note_table = "dose" and note_thing = $id ORDER BY created);
RETURN $results;
};
