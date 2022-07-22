DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'story_user_relationship') THEN
        CREATE TYPE story_user_relationship AS ENUM ('author', 'comissioner', 'dedicated', 'bookmaker', 'follower');
    END IF;
END $$;
