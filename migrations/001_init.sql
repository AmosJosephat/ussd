-- Create user_status enum
CREATE TYPE user_status AS ENUM ('active', 'inactive', 'suspended');

-- Create transaction_status enum
CREATE TYPE transaction_status AS ENUM ('pending', 'completed', 'failed', 'cancelled');

-- Create users table
CREATE TABLE users (
    id UUID PRIMARY KEY,
    phone_number VARCHAR(20) UNIQUE NOT NULL,
    name VARCHAR(255),
    email VARCHAR(255),
    language VARCHAR(10) NOT NULL DEFAULT 'en',
    status user_status NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index on phone_number for faster lookups
CREATE INDEX idx_users_phone_number ON users(phone_number);

-- Create transactions table
CREATE TABLE transactions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    session_id VARCHAR(255) NOT NULL,
    transaction_type VARCHAR(50) NOT NULL,
    amount DECIMAL(15, 2),
    status transaction_status NOT NULL DEFAULT 'pending',
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes on transactions
CREATE INDEX idx_transactions_user_id ON transactions(user_id);
CREATE INDEX idx_transactions_session_id ON transactions(session_id);
CREATE INDEX idx_transactions_created_at ON transactions(created_at DESC);
CREATE INDEX idx_transactions_status ON transactions(status);

-- Create analytics view
CREATE VIEW session_analytics AS
SELECT
    date_trunc('day', created_at) as date,
    COUNT(*) as session_count,
    COUNT(DISTINCT user_id) as unique_users,
    AVG(amount) as avg_amount
FROM transactions
GROUP BY date_trunc('day', created_at)
ORDER BY date DESC;

-- Create function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create trigger for users table
CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Insert sample data (optional, for testing)
-- INSERT INTO users (id, phone_number, name, language) VALUES
--     ('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '+1234567890', 'Test User', 'en');
