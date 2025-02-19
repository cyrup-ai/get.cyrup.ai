# SQL Development Conventions

## Table of Contents
- [Naming Conventions](#naming-conventions)
- [Query Structure](#query-structure)
- [Schema Design](#schema-design)
- [Performance](#performance)
- [Security](#security)
- [Version Control](#version-control)
- [Documentation](#documentation)

## Naming Conventions

### General Rules
- Use snake_case for all names
- Use full English words, not abbreviations
- Be descriptive and consistent
- Prefix temp tables with `tmp_`
- Prefix views with `v_`
- Prefix materialized views with `mv_`

### Tables
- Use plural nouns (users, orders, products)
- Join tables: combine both table names (order_items)
- Avoid reserved words and special characters

### Columns
- Primary keys: `id` or `table_name_id`
- Foreign keys: `referenced_table_name_id`
- Boolean columns: prefix with `is_`, `has_`, or `can_`
- Timestamp columns:
  - `created_at`
  - `updated_at`
  - `deleted_at` (for soft deletes)
  - `processed_at`
  - `completed_at`

### Indexes
- Format: `idx_table_name_column_name`
- Unique: `unq_table_name_column_name`
- Foreign keys: `fk_table_name_ref_table`

## Query Structure

### Formatting
```sql
SELECT DISTINCT
    u.first_name,
    u.last_name,
    o.order_date
FROM
    users u
    INNER JOIN orders o ON u.id = o.user_id
WHERE
    o.status = 'completed'
    AND o.created_at >= CURRENT_DATE - INTERVAL '30 days'
GROUP BY
    u.first_name,
    u.last_name,
    o.order_date
HAVING
    COUNT(*) > 1
ORDER BY
    o.order_date DESC
LIMIT 100;
```

### Best Practices
- One statement per line
- Indent subqueries and CTEs
- Align ON clauses with their JOIN
- Use table aliases that make sense
- UPPERCASE keywords for visibility
- Break long lines at logical points
- Use CTEs for complex queries

### Joins
- Explicitly specify JOIN type (INNER, LEFT, etc.)
- Place referenced table on right side of JOIN
- Always use table aliases for complex queries
- Document JOIN purposes in comments

## Schema Design

### Tables
- Always include:
  - Primary key (usually auto-incrementing)
  - `created_at` timestamp
  - `updated_at` timestamp
- Consider including:
  - `deleted_at` for soft deletes
  - `version` for optimistic locking
  - `created_by`, `updated_by` for audit

### Constraints
```sql
CREATE TABLE orders (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    status VARCHAR(50) NOT NULL,
    total_amount DECIMAL(10,2) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT fk_orders_user
        FOREIGN KEY (user_id)
        REFERENCES users(id),
    CONSTRAINT chk_orders_total_amount
        CHECK (total_amount >= 0),
    CONSTRAINT chk_orders_status
        CHECK (status IN ('pending', 'completed', 'cancelled'))
);
```

### Indexes
- Index foreign keys
- Index frequently filtered columns
- Consider partial indexes for performance
- Document index justification in comments

## Performance

### Query Optimization
- Use EXPLAIN ANALYZE for query planning
- Avoid SELECT *
- Use appropriate JOIN types
- Consider materialized views for complex queries
- Use indexes effectively

### Common Problems
- N+1 queries
- Cartesian products
- Unnecessary subqueries
- Missing indexes
- Over-indexing

### Batch Processing
```sql
WITH batch AS (
    SELECT id
    FROM orders
    WHERE processed_at IS NULL
    ORDER BY created_at
    LIMIT 1000
    FOR UPDATE SKIP LOCKED
)
UPDATE orders o
SET processed_at = CURRENT_TIMESTAMP
FROM batch b
WHERE o.id = b.id;
```

## Security

### Access Control
- Use role-based access control (RBAC)
- Principle of least privilege
- Regular permission audits
- Document access patterns

### SQL Injection Prevention
- Use parameterized queries
- Validate and sanitize inputs
- Escape special characters
- Use ORMs when possible

### Sensitive Data
- Encrypt sensitive columns
- Use appropriate data types
- Implement column-level security
- Audit access to sensitive data

## Version Control

### Migration Files
```sql
-- V1.0.0__create_users_table.sql
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- V1.0.1__add_user_name_columns.sql
ALTER TABLE users
ADD COLUMN first_name VARCHAR(100),
ADD COLUMN last_name VARCHAR(100);
```

### Version Control Best Practices
- One change per migration
- Never modify committed migrations
- Include both up and down migrations
- Test migrations in staging
- Document breaking changes

## Documentation

### Inline Documentation
```sql
-- Calculate the total revenue per product category
-- Includes only completed orders from the last 30 days
-- Revenue is calculated before discounts and taxes
WITH monthly_sales AS (
    SELECT
        p.category_id,
        SUM(oi.quantity * oi.unit_price) as revenue
    FROM order_items oi
    JOIN products p ON p.id = oi.product_id
    JOIN orders o ON o.id = oi.order_id
    WHERE
        o.status = 'completed'
        AND o.created_at >= CURRENT_DATE - INTERVAL '30 days'
    GROUP BY p.category_id
)
SELECT
    c.name as category_name,
    ms.revenue
FROM monthly_sales ms
JOIN categories c ON c.id = ms.category_id
ORDER BY ms.revenue DESC;
```

### Schema Documentation
- Document table purposes
- Explain column meanings
- Note business rules
- Include example queries
- Document dependencies

### Performance Documentation
- Document expected query patterns
- Note index usage expectations
- Document optimization decisions
- Include benchmark results

---
