-- Add migration script here
CREATE TABLE IF NOT EXISTS user (
	user_id VARCHAR(255) PRIMARY KEY COMMENT 'ユーザーID',
	name VARCHAR(255) NOT NULL COMMENT 'ユーザー名',
	birthday INT NOT NULL COMMENT '生年月日',
	email VARCHAR(255) NOT NULL COMMENT 'メールアドレス',
	blood_type TINYINT DEFAULT 0 COMMENT '血液型(0: 不明, 1: A, 2: B, 3: O, 4: AB)',
	created_at INT NOT NULL COMMENT '登録日時',
	updated_at INT NOT NULL COMMENT '更新日時',
	is_deleted BOOLEAN DEFAULT FALSE COMMENT '削除フラグ'
);