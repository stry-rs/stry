DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'story_tag_level') THEN
        CREATE TYPE story_tag_level AS ENUM ('major', 'minor');
    END IF;
END $$;
