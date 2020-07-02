create table members (
	visitorid varchar(128) primary key,
	bucket int default 0,
	streak_total int default 0,
	streak_bucket int default 0,
	balance float default 0,

	email varchar(255)
);

create table scansessions ( 
	uuid uuid primary key,
	visitorid varchar(128) references members(visitorid) not null,
	started timestamp,
	stopped timestamp,
	status varchar(16)
);

create table anodes (
	label varchar(32) primary key,
	description varchar(255)
);

create table scans (
	scansession uuid references scansessions(uuid) not null,
	anode varchar(128) references anodes(label) not null,
	tstamp timestamp
);

create table events (
	uuid uuid primary key,
	visitorid varchar(128) references members(visitorid),
	tstamp timestamp,
	label varchar(16),
	data jsonb
);
