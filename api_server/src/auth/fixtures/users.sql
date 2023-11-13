INSERT INTO users
(id, username, password, is_admin, is_trusted, works_for, email, can_edit_departures)
VALUES (1, 'admin', '$pbkdf2-sha256$i=600000,l=32$+vjDQdvAJ9u5mTENXMfUcw$opD9UivlJvbKO0lV8RHrD+LChE9/+6lIuE2P04RlQAM',
        true, true, NULL, 'admin@users.com', true),
       (2, 'user', '$pbkdf2-sha256$i=600000,l=32$+vjDQdvAJ9u5mTENXMfUcw$opD9UivlJvbKO0lV8RHrD+LChE9/+6lIuE2P04RlQAM',
        false, false, NULL, 'user1@users.com', false);