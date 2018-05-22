declare
begin
    execute immediate q'<ALTER TABLE posts ADD CONSTRAINT posts_user_id_fkey FOREIGN KEY (user_id) REFERENCES users DEFERRABLE>';
    execute immediate q'<ALTER TABLE comments ADD CONSTRAINT comments_post_id_fkey FOREIGN KEY (post_id) REFERENCES posts DEFERRABLE>';
    execute immediate q'<ALTER TABLE followings ADD CONSTRAINT followings_user_id_fkey FOREIGN KEY (user_id) REFERENCES users DEFERRABLE>';
    execute immediate q'<ALTER TABLE followings ADD CONSTRAINT followings_post_id_fkey FOREIGN KEY (post_id) REFERENCES posts DEFERRABLE>';
    execute immediate q'<ALTER TABLE likes ADD CONSTRAINT likes_comment_id_fkey FOREIGN KEY (comment_id) REFERENCES comments DEFERRABLE>';
    execute immediate q'<ALTER TABLE likes ADD CONSTRAINT likes_user_id_fkey FOREIGN KEY (user_id) REFERENCES users DEFERRABLE>';

end;
