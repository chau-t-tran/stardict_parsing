CREATE TABLE entries (
	id INT PRIMARY KEY AUTO_INCREMENT
);

CREATE TABLE senses (
	id INT PRIMARY KEY AUTO_INCREMENT,
	entry_id INT,
	FOREIGN KEY (entry_id) REFERENCES entries(id)
);

CREATE TABLE definitions (
	id INT PRIMARY KEY AUTO_INCREMENT,
	sense_id INT,
	FOREIGN KEY (sense_id) REFERENCES senses(id)
);

CREATE TABLE part_of_speeches (
	id INT PRIMARY KEY AUTO_INCREMENT,
	sense_id INT,
	FOREIGN KEY (sense_id) REFERENCES senses(id)
);

CREATE TABLE sentences (
	id INT PRIMARY KEY AUTO_INCREMENT,
	sense_id INT,
	FOREIGN KEY (sense_id) REFERENCES senses(id)
);

CREATE TABLE engs (
	id INT PRIMARY KEY AUTO_INCREMENT,
	sentence_id INT,
	FOREIGN KEY (sentence_id) REFERENCES sentences(id)
);

CREATE TABLE viets (
	id INT PRIMARY KEY AUTO_INCREMENT,
	sentence_id INT,
	FOREIGN KEY (sentence_id) REFERENCES sentences(id)
);

CREATE TABLE entry_text_audio_pairs (
	id INT PRIMARY KEY AUTO_INCREMENT,
	text TEXT,
	audio_url TEXT,
	entry_id INT,
	FOREIGN KEY (entry_id) REFERENCES entries(id)
);

CREATE TABLE definition_text_audio_pairs (
	id INT PRIMARY KEY AUTO_INCREMENT,
	text TEXT,
	audio_url TEXT,
	definition_id INT,
	FOREIGN KEY (definition_id) REFERENCES definitions(id)
);

CREATE TABLE part_of_speech_text_audio_pairs (
	id INT PRIMARY KEY AUTO_INCREMENT,
	text TEXT,
	audio_url TEXT,
	part_of_speech_id INT,
	FOREIGN KEY (part_of_speech_id) REFERENCES part_of_speeches(id)
);

CREATE TABLE eng_text_audio_pairs (
	id INT PRIMARY KEY AUTO_INCREMENT,
	text TEXT,
	audio_url TEXT,
	eng_id INT,
	FOREIGN KEY (eng_id) REFERENCES engs(id)
);

CREATE TABLE viet_text_audio_pairs (
	id INT PRIMARY KEY AUTO_INCREMENT,
	text TEXT,
	audio_url TEXT,
	viet_id INT,
	FOREIGN KEY (viet_id) REFERENCES viets(id)
);
