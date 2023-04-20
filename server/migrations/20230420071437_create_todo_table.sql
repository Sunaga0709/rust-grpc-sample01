-- Add migration script here
CREATE TABLE IF NOT EXISTS todo (
	todo_id VARCHAR(255) PRIMARY KEY COMMENT 'TODO ID',
	user_id VARCHAR(255) NOT NULL COMMENT 'ユーザーID',
	title VARCHAR(255) NOT NULL COMMENT 'タイトル',
	description VARCHAR(255) NOT NULL DEFAULT '' COMMENT '説明',
	status TINYINT NOT NULL DEFAULT 0 COMMENT 'ステータス(0: 未着手, 1: 着手中, 2: 完了, 3: 中断)',
	created_at INT NOT NULL COMMENT '登録日時',
	updated_at INT NOT NULL COMMENT '更新日時',
	is_deleted BOOLEAN DEFAULT FALSE COMMENT '削除フラグ'
);