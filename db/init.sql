-- ==========================
-- 文章
-- ==========================

CREATE TABLE blogs
(
    id          BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,

    folder_id   BIGINT       NOT NULL,

    title       VARCHAR(200) NOT NULL,

    description TEXT         NOT NULL,

    cover_url   TEXT,

    author      VARCHAR(100) NOT NULL,

    content     TEXT         NOT NULL,
    is_pinned   BOOLEAN      NOT NULL DEFAULT FALSE,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_blog_folder
        FOREIGN KEY (folder_id)
            REFERENCES blog_folders (id)
            ON DELETE CASCADE
);

INSERT INTO blogs(folder_id,
                  title,
                  description,
                  author,
                  content)
VALUES (1,
        'Axum路由系统',
        'Axum路由注册笔记',
        'admin',
        '# Axum Router\n\n记录Axum路由系统...'),
       (1,
        'SQLx使用总结',
        'SQLx连接池与查询',
        'admin',
        '# SQLx\n\n连接池配置...'),
       (3,
        'PostgreSQL索引',
        'BTree索引学习记录',
        'admin',
        '# PostgreSQL Index\n\n...'),
       (6,
        '摄影参数记录',
        '快门光圈ISO学习',
        'admin',
        '# 摄影\n\n...');
