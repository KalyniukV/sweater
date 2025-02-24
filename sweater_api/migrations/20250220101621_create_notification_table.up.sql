CREATE TABLE notifications (
                               id UUID NOT NULL PRIMARY KEY,
                               user_id UUID NOT NULL,
                               text TEXT NOT NULL,
                               created_at TIMESTAMP NOT NULL
);
