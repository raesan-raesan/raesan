CREATE TABLE classes(
	id TEXT PRIMARY KEY NOT NULL,
	name INTEGER UNIQUE NOT NULL,
	created_at INTEGER NOT NULL,
	updated_at INTEGER NOT NULL
);

CREATE TABLE subjects(
	id TEXT PRIMARY KEY NOT NULL,
	name TEXT NOT NULL,
	display_name TEXT NOT NULL UNIQUE,
	class_id TEXT NOT NULL,
	class_name INTEGER NOT NULL,
	created_at INTEGER NOT NULL,
	updated_at INTEGER NOT NULL,
	FOREIGN KEY  (class_id) REFERENCES classes(id) ON DELETE CASCADE
);

CREATE TABLE chapters(
	id TEXT PRIMARY KEY NOT NULL,
	name TEXT NOT NULL,
	display_name TEXT NOT NULL,
	subject_id TEXT NOT NULL,
	subject_name TEXT NOT NULL,
	class_name INTEGER NOT NULL,
	created_at INTEGER NOT NULL,
	updated_at INTEGER NOT NULL,
	FOREIGN KEY  (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);

CREATE TABLE questions(
	id TEXT PRIMARY KEY NOT NULL,
	body TEXT UNIQUE NOT NULL,
	chapter_name TEXT NOT NULL,
	subject_name TEXT NOT NULL,
	class_name INTEGER NOT NULL,
	chapter_id TEXT NOT NULL,
	created_at INTEGER NOT NULL,
	updated_at INTEGER NOT NULL,
	FOREIGN KEY  (chapter_id) REFERENCES chapters(id) ON DELETE CASCADE
);
