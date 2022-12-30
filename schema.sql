create table hjärnor(
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	hjärnannamn VARCHAR(255) NOT NULL,
	lösenord VARCHAR(255) NOT NULL,
	födelsedag TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
create table fantasifoster(
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	titel VARCHAR(255) NOT NULL, 
	innehåll TEXT NOT NULL,
	födelsedag TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	uppfinnare UUID,
	CONSTRAINT fk_hjärnor FOREIGN KEY(uppfinnare) REFERENCES hjärnor(id)
	)
	
	