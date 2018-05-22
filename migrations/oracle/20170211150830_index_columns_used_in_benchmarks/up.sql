begin
execute immediate q'<CREATE INDEX users_hair_color ON users (hair_color)>';
execute immediate q'<CREATE INDEX posts_user_id ON posts (user_id)>';

end;
