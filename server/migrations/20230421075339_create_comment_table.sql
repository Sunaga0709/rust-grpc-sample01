-- Add migration script here
CREATE TABLE IF NOT EXISTS comment (
	comment_id VARCHAR(255) PRIMARY KEY COMMENT 'コメントID',
	todo_id VARCHAR(255) NOT NULL COMMENT 'todo ID',
	text VARCHAR(255) NOT NULL COMMENT 'コメント本文',
	created_at INT NOT NULL COMMENT '登録日時',
	is_deleted BOOLEAN NOT NULL DEFAULT FALSE COMMENT '削除フラグ'
);