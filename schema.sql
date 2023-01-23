create table brains(
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	brainname VARCHAR(255) NOT NULL UNIQUE,
	password VARCHAR(255) NOT NULL,
	birthdate TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	lastupdatedate TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
create table brainfarts(
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	title VARCHAR(255) NOT NULL, 
	content TEXT NOT NULL,
	birthdate TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	mastermind UUID,
	CONSTRAINT fk_brains FOREIGN KEY(mastermind) REFERENCES brains(id)
	);

create table mindsblownbyfarts(
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	brainfartid UUID,
	brainid UUID,
	explosion bool,
	CONSTRAINT fk_brain FOREIGN KEY(brainid) REFERENCES brains(id),
	CONSTRAINT fk_brainfart FOREIGN KEY(brainfartid) REFERENCES brainfarts(id)
);
create table hallucinatedfarts(
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	brainfartid UUID,
	brainid UUID,
	hallucinationdate TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	CONSTRAINT fk_brain FOREIGN KEY(brainid) REFERENCES brains(id),
	CONSTRAINT fk_brainfart FOREIGN KEY(brainfartid) REFERENCES brainfarts(id)
);