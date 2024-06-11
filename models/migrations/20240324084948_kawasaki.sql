-- 参与者基本信息
CREATE TABLE IF NOT EXISTS participant (
    id INT PRIMARY KEY,
    name VARCHAR(20) NOT NULL,
    birth_date DATE NOT NULL,
    gender VARCHAR(1) NOT NULL,
    weight REAL NOT NULL,
    height REAL NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 分组信息
CREATE TABLE IF NOT EXISTS enrollgroup (
    id SERIAL PRIMARY KEY,
    name VARCHAR(10) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 档案信息
CREATE TABLE IF NOT EXISTS document (
    id INT PRIMARY KEY,
    participant_id INT NOT NULL,
    group_id INT NOT NULL,
    -- 入院时间
    in_date DATE NOT NULL,
    -- 创建时间
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT FK_doc_group FOREIGN KEY (group_id) REFERENCES enrollgroup(id),
    CONSTRAINT FK_doc_par FOREIGN KEY (participant_id) REFERENCES participant(id),
    UNIQUE (participant_id, group_id)
);

-- 记录患者症状
CREATE TABLE IF NOT EXISTS physical (
    id SERIAL PRIMARY KEY,
    document_id INT NOT NULL,
    fever_days INT NOT NULL,
    red_eyes BOOLEAN DEFAULT FALSE,
    lips_tongue BOOLEAN DEFAULT FALSE,
    lymph_nodes BOOLEAN DEFAULT FALSE,
    rash BOOLEAN DEFAULT FALSE,
    hands_feet BOOLEAN DEFAULT FALSE,
    relapse BOOLEAN DEFAULT FALSE,
    resistance BOOLEAN DEFAULT FALSE,
    CONSTRAINT FK_physical FOREIGN KEY (document_id) REFERENCES document(id)
);

CREATE TABLE IF NOT EXISTS bloodtest (
    id SERIAL PRIMARY KEY,
    document_id INT NOT NULL,
    date DATE NOT NULL,
    wbc REAL NOT NULL,
    ne REAL NOT NULL,
    ly REAL NOT NULL,
    mo REAL NOT NULL,
    rbc REAL NOT NULL,
    hgb REAL NOT NULL,
    plt REAL NOT NULL,
    CONSTRAINT FK_bloodtest_document FOREIGN KEY (document_id) REFERENCES document(id)
);

CREATE TABLE IF NOT EXISTS liverfunction (
    id SERIAL PRIMARY KEY,
    document_id INT NOT NULL,
    date DATE NOT NULL,
    ast REAL NOT NULL,
    alt REAL NOT NULL,
    alb REAL NOT NULL,
    CONSTRAINT FK_liverfunction_document FOREIGN KEY (document_id) REFERENCES document(id)
);

CREATE TABLE IF NOT EXISTS othertest (
    id SERIAL PRIMARY KEY,
    document_id INT NOT NULL,
    date DATE NOT NULL,
    pct REAL,
    crp REAL,
    esr REAL,
    CONSTRAINT FK_othertest_document FOREIGN KEY (document_id) REFERENCES document(id)
);

CREATE TABLE IF NOT EXISTS echocardiography (
    id SERIAL PRIMARY KEY,
    document_id INT NOT NULL,
    date DATE NOT NULL,
    rca REAL NOT NULL,
    lmca REAL NOT NULL,
    lad REAL,
    lcx REAL,
    CONSTRAINT FK_echocardiography_document FOREIGN KEY (document_id) REFERENCES document(id)
);

CREATE TABLE IF NOT EXISTS samples (
    id SERIAL PRIMARY KEY,
    document_id INT NOT NULL,
    date DATE NOT NULL,
    type VARCHAR(1) NOT NULL,
    label VARCHAR(10) NOT NULL UNIQUE,
    status VARCHAR(1) NOT NULL,
    note TEXT,
    CONSTRAINT FK_samples_document FOREIGN KEY (document_id) REFERENCES document(id)
);
