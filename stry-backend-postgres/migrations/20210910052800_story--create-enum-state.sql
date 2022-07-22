DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'story_state') THEN
        CREATE TYPE story_state AS ENUM ('completed', 'in-progress', 'hiatus', 'abandoned');
    END IF;
END $$;
