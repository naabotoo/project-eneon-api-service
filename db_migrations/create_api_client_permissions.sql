-- Create UUID extension if not exists
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create permissions table
CREATE TABLE permissions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    permission_key VARCHAR(255) NOT NULL UNIQUE,
    created_on TIMESTAMP DEFAULT now(),
    updated_on TIMESTAMP DEFAULT now()
);

-- Create roles table
CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    role_id VARCHAR(255) NOT NULL UNIQUE,
    created_on TIMESTAMP DEFAULT now(),
    updated_on TIMESTAMP DEFAULT now()
);

-- Create role_permissions junction table
CREATE TABLE role_permissions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    role_id UUID NOT NULL,
    permission_id UUID NOT NULL,
    created_on TIMESTAMP DEFAULT now(),
    updated_on TIMESTAMP DEFAULT now(),
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
    FOREIGN KEY (permission_id) REFERENCES permissions(id) ON DELETE CASCADE,
    UNIQUE(role_id, permission_id)
);

-- Create clients table
CREATE TABLE client (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    client_id VARCHAR(255) NOT NULL UNIQUE,
    client_secret VARCHAR(500) NOT NULL,
    is_active BOOLEAN DEFAULT false,
    created_on TIMESTAMP DEFAULT now(),
    updated_on TIMESTAMP DEFAULT now()
);

-- Create client_roles junction table
CREATE TABLE client_roles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    client_id UUID NOT NULL,
    role_id UUID NOT NULL,
    created_on TIMESTAMP DEFAULT now(),
    updated_on TIMESTAMP DEFAULT now(),
    FOREIGN KEY (client_id) REFERENCES client(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
    UNIQUE(client_id, role_id)
);

-- Create clients_allowed table
CREATE TABLE clients_allowed (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    client_id UUID NOT NULL,
    permission_key JSONB NOT NULL,
    created_on TIMESTAMP DEFAULT now(),
    updated_on TIMESTAMP DEFAULT now(),
    FOREIGN KEY (client_id) REFERENCES client(id) ON DELETE CASCADE
);

-- Create clients_denied table
CREATE TABLE clients_denied (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    client_id UUID NOT NULL,
    permission_key JSONB NOT NULL,
    created_on TIMESTAMP DEFAULT now(),
    updated_on TIMESTAMP DEFAULT now(),
    FOREIGN KEY (client_id) REFERENCES client(id) ON DELETE CASCADE
);

-- Create indexes for better query performance
CREATE INDEX idx_role_permissions_role_id ON role_permissions(role_id);
CREATE INDEX idx_role_permissions_permission_id ON role_permissions(permission_id);
CREATE INDEX idx_client_roles_client_id ON client_roles(client_id);
CREATE INDEX idx_client_roles_role_id ON client_roles(role_id);
CREATE INDEX idx_clients_allowed_client_id ON clients_allowed(client_id);
CREATE INDEX idx_clients_denied_client_id ON clients_denied(client_id);
