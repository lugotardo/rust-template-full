-- Inserir alguns usuários de exemplo
INSERT INTO users (name, email, active) VALUES 
    ('Alice Silva', 'alice@example.com', true),
    ('Bob Santos', 'bob@example.com', true),
    ('Charlie Costa', 'charlie@example.com', false)
ON CONFLICT (email) DO NOTHING;
