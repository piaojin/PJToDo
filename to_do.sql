CREATE TABLE `todo` (
	`id`	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`title`	TEXT NOT NULL DEFAULT "",
	`content`	TEXT NOT NULL DEFAULT "",
	`due_time`	TEXT NOT NULL,
	`remind_time`	TEXT NOT NULL,
	`create_time`	TEXT NOT NULL,
	`update_time`	TEXT NOT NULL DEFAULT "",
	`to_do_tag_id`	INTEGER NOT NULL DEFAULT -1,
	`to_do_type_id`	INTEGER NOT NULL DEFAULT -1,
	`state`	TEXT NOT NULL DEFAULT "Determined",
	CONSTRAINT `fk_to_do_tag` FOREIGN KEY(`to_do_tag_id`) REFERENCES `todotag`(`id`),
	CONSTRAINT `fk_to_do_type` FOREIGN KEY(`to_do_type_id`) REFERENCES `todotype`(`id`)
);