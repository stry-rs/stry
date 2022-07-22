DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'story_rating') THEN
        CREATE TYPE story_rating AS ENUM ('explicit', 'mature', 'teen', 'general');
    END IF;
END $$;
