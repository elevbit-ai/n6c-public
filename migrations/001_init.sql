-- SPECTER-NET Database Schema
-- PostgreSQL

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Sensors table
CREATE TABLE IF NOT EXISTS sensors (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    location VARCHAR(512) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'offline',
    last_seen TIMESTAMPTZ,
    center_frequency_hz BIGINT NOT NULL,
    sample_rate INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Radios table
CREATE TABLE IF NOT EXISTS radios (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    model VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'offline',
    current_channel INTEGER NOT NULL DEFAULT 1,
    current_frequency_hz BIGINT NOT NULL,
    last_seen TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Authorized channels table
CREATE TABLE IF NOT EXISTS authorized_channels (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    radio_id UUID NOT NULL REFERENCES radios(id) ON DELETE CASCADE,
    channel INTEGER NOT NULL,
    frequency_hz BIGINT NOT NULL,
    bandwidth_hz BIGINT NOT NULL,
    max_power_dbm DOUBLE PRECISION NOT NULL,
    label VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(radio_id, channel)
);

-- Spectrum measurements (time-partitioned)
CREATE TABLE IF NOT EXISTS spectrum_measurements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    sensor_id UUID NOT NULL REFERENCES sensors(id),
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    center_frequency_hz BIGINT NOT NULL,
    bandwidth_hz BIGINT NOT NULL,
    sample_rate INTEGER NOT NULL,
    fft_size INTEGER NOT NULL,
    noise_floor_dbm DOUBLE PRECISION NOT NULL,
    peak_dbm DOUBLE PRECISION NOT NULL,
    mean_dbm DOUBLE PRECISION NOT NULL,
    snr_db DOUBLE PRECISION NOT NULL,
    occupancy DOUBLE PRECISION NOT NULL,
    psd JSONB
);

CREATE INDEX IF NOT EXISTS idx_measurements_sensor_time
    ON spectrum_measurements(sensor_id, timestamp DESC);

CREATE INDEX IF NOT EXISTS idx_measurements_time
    ON spectrum_measurements(timestamp DESC);

-- RF Events table
CREATE TABLE IF NOT EXISTS rf_events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    sensor_id UUID NOT NULL REFERENCES sensors(id),
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    event_type VARCHAR(100) NOT NULL,
    confidence DOUBLE PRECISION NOT NULL,
    center_frequency_hz BIGINT NOT NULL,
    bandwidth_hz BIGINT NOT NULL,
    duration_secs DOUBLE PRECISION NOT NULL DEFAULT 0,
    noise_floor_delta_db DOUBLE PRECISION NOT NULL DEFAULT 0,
    occupancy DOUBLE PRECISION NOT NULL DEFAULT 0,
    snr_drop_db DOUBLE PRECISION NOT NULL DEFAULT 0,
    description TEXT,
    acknowledged BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_events_sensor_time
    ON rf_events(sensor_id, timestamp DESC);

CREATE INDEX IF NOT EXISTS idx_events_type
    ON rf_events(event_type);

-- Channel changes table
CREATE TABLE IF NOT EXISTS channel_changes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    radio_id UUID NOT NULL REFERENCES radios(id),
    sensor_id UUID NOT NULL REFERENCES sensors(id),
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    from_channel INTEGER NOT NULL,
    to_channel INTEGER NOT NULL,
    from_frequency_hz BIGINT NOT NULL,
    to_frequency_hz BIGINT NOT NULL,
    reason TEXT NOT NULL,
    state VARCHAR(50) NOT NULL,
    success BOOLEAN,
    rolled_back BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_changes_radio_time
    ON channel_changes(radio_id, timestamp DESC);

-- Alerts table
CREATE TABLE IF NOT EXISTS alerts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    level VARCHAR(50) NOT NULL,
    source VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    acknowledged BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_alerts_level
    ON alerts(level, timestamp DESC);

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(255) UNIQUE NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'viewer',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login TIMESTAMPTZ
);

-- Audit log table
CREATE TABLE IF NOT EXISTS audit_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id UUID REFERENCES users(id),
    action VARCHAR(255) NOT NULL,
    resource_type VARCHAR(100) NOT NULL,
    resource_id UUID,
    details JSONB,
    ip_address INET,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_audit_time
    ON audit_log(timestamp DESC);

CREATE INDEX IF NOT EXISTS idx_audit_user
    ON audit_log(user_id, timestamp DESC);

-- System config table
CREATE TABLE IF NOT EXISTS system_config (
    key VARCHAR(255) PRIMARY KEY,
    value JSONB NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
