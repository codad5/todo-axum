use super::query::BindType;


pub fn vec_bind_type_to_string(bind_values: &Vec<BindType>) -> Vec<String> {
    let mut values = Vec::new();
    for bind_value in bind_values {
        match bind_value {
            BindType::Int(value) => {
                values.push(value.to_string());
            }
            BindType::String(value) => {
                values.push(format!("'{}'", value));
            }
            BindType::Bool(value) => {
                values.push(value.to_string());
            }
            BindType::Float(value) => {
                values.push(value.to_string());
            }
            BindType::PreparedStatement => {
                values.push("?".to_string());
            }
        }
    }
    values
}

pub enum MySQlAction {
    Select(String),
    Insert((Vec<String>, Vec<BindType>)),
    Update(Vec<(String, BindType)>),
    Delete,
    None,

}

impl MySQlAction {
    pub fn to_string(&self, table: &str) -> String {
        match self {
            MySQlAction::Select(fields) => format!("SELECT {} FROM {}", fields, table),
            MySQlAction::Insert((fields, value)) => format!("INSERT INTO {} ({}) VALUES ({})", table, fields.join(", "), vec_bind_type_to_string(value).join(", ")),
            MySQlAction::Update(fields) => {
                let mut query = format!("UPDATE {} SET", table);
                for (index, field) in fields.iter().enumerate() {
                    query = format!("{} {} = {}", query, field.0, match &field.1 {
                        BindType::Int(value) => value.to_string(),
                        BindType::String(value) => format!("'{}'", value),
                        BindType::Bool(value) => value.to_string(),
                        BindType::Float(value) => value.to_string(),
                        BindType::PreparedStatement => "?".to_string(),
                    });
                    if index < fields.len() - 1 {
                        query = format!("{},", query);
                    }
                }
                query
            },
            MySQlAction::Delete => format!("DELETE FROM {}", table),
            _ => "".to_string(),
        }
    }
}


pub enum MySQLJoin {
    Inner,
    Left,
    Right,
    Full,
}

pub enum MySQLLogicalOperator {
    And,
    Or,
}

pub enum MySQLArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub enum LikeOperator {
    StartsWith(String),
    EndsWith(String),
    Contains(String),
    Exact(String),
    None,
}

impl LikeOperator {
    pub fn to_string(&self) -> String {
        match self {
            LikeOperator::StartsWith(value) => format!("{}%", value),
            LikeOperator::EndsWith(value) => format!("%{}", value),
            LikeOperator::Contains(value) => format!("%{}%", value),
            LikeOperator::Exact(value) => value.clone(),
            LikeOperator::None => "".to_string(),
        }
    }
}

pub enum InCondition {
    Vector(Vec<String>),
    Query(QueryBuilder),
}



pub enum MySQLCondition {
    Equal(BindType),
    NotEqual(BindType),
    GreaterThan(i32),
    LessThan(i32),
    GreaterThanEqual(i32),
    LessThanEqual(i32),
    Like(LikeOperator),
    NotLike(LikeOperator),
    In(InCondition),
    NotIn(InCondition),
    Between(i32, i32),
    NotBetween(i32, i32),
    IsNull,
    IsNotNull,
}

impl MySQLCondition {
    pub fn to_string(&self, prepared_stmt: bool) -> String {
        match self {
            MySQLCondition::Equal(value) => format!("= {}", if prepared_stmt {BindType::PreparedStatement } else { value.to_owned() }),
            MySQLCondition::NotEqual(value) => format!("!= {}", if prepared_stmt { BindType::PreparedStatement } else { value.to_owned() }),
            MySQLCondition::GreaterThan(value) => format!("> {}", if prepared_stmt { BindType::PreparedStatement} else { BindType::Int(value.to_owned()) }),
            MySQLCondition::LessThan(value) => format!("< {}", if prepared_stmt { BindType::PreparedStatement} else { BindType::Int(value.to_owned()) }),
            MySQLCondition::GreaterThanEqual(value) => format!(">= {}", if prepared_stmt { BindType::PreparedStatement} else { BindType::Int(value.to_owned()) }),
            MySQLCondition::LessThanEqual(value) => format!("<= {}", if prepared_stmt { BindType::PreparedStatement} else { BindType::Int(value.to_owned()) }),
            MySQLCondition::Like(value) => format!("LIKE {}", value.to_string()),
            MySQLCondition::NotLike(value) => format!("NOT LIKE {}", value.to_string()),
            MySQLCondition::In(value) => match value {
                InCondition::Vector(values) => format!("IN ({})", values.join(", ")),
                InCondition::Query(query) => format!("IN ({})", query.build()),
            },
            MySQLCondition::NotIn(value) => match value {
                InCondition::Vector(values) => format!("NOT IN ({})", values.join(", ")),
                InCondition::Query(query) => format!("NOT IN ({})", query.build()),
            },
            MySQLCondition::Between(value1, value2) => format!("BETWEEN {} AND {}", if prepared_stmt { BindType::PreparedStatement } else { BindType::Int(value1.to_owned()) }, if prepared_stmt { BindType::PreparedStatement } else { BindType::Int(value2.to_owned()) }),
            MySQLCondition::NotBetween(value1, value2) => format!("NOT BETWEEN {} AND {}", if prepared_stmt { BindType::PreparedStatement } else { BindType::Int(value1.to_owned()) }, if prepared_stmt { BindType::PreparedStatement } else { BindType::Int(value2.to_owned()) }),
            MySQLCondition::IsNull => "IS NULL".to_string(),
            MySQLCondition::IsNotNull => "IS NOT NULL".to_string(),
        }
    }
}

pub enum MySQLOrder {
    Asc,
    Desc,
}

pub struct QueryBuilder {
    table: String,
    action : MySQlAction,
    // join of vec of table to join and the type of join and field condition to join
    joins : Vec<(String, MySQLJoin, String)>,
    // where condition
    where_condition : Vec<(String, MySQLCondition)>, // field, condition, value
    // group by
    group_by : Vec<String>,
    // having
    having : Vec<(String, MySQLCondition, String)>, // field, condition, value
    // order by
    order_by : Vec<(String, MySQLOrder)>, // field, order
    // limit
    limit : Option<u32>,
    prepared_stmt : bool,


}

impl QueryBuilder {
    pub fn new(table: &str) -> Self {
        Self { 
            table : table.to_string(),
            action : MySQlAction::None ,
            joins : Vec::new(),
            where_condition : Vec::new(),
            group_by : Vec::new(),
            having : Vec::new(),
            order_by : Vec::new(),
            limit : None,
            prepared_stmt : true,
        }
    }

    pub fn prepared_stmt(&mut self, prepared_stmt: bool) -> &mut Self {
        self.prepared_stmt = prepared_stmt;
        self
    }

    pub fn select(&mut self, fields: Vec<String>)  -> &mut Self {
        let fields = match fields.len() {
            0 => "*".to_string(),
            _ => fields.join(", "),
        };
        self.action = MySQlAction::Select(fields);
        self
    }

    pub fn insert(&mut self, fields: Vec<String>, values: Vec<BindType>) -> &mut Self {
        self.action = MySQlAction::Insert((fields, if self.prepared_stmt { vec![BindType::PreparedStatement; values.len()] } else { values }));
        self
    }

    pub fn delete(&mut self) -> &mut Self {
        self.action = MySQlAction::Delete;
        self
    }

    pub fn update(&mut self, fields: Vec<(String, BindType)>) -> &mut Self {
        let mut update_fields = Vec::new();
        for field in fields {
            update_fields.push((field.0, if self.prepared_stmt { BindType::PreparedStatement } else { field.1 }));
        }
        self.action = MySQlAction::Update(update_fields);
        self
    }

    pub fn join(&mut self, table: String, join: MySQLJoin, field: String) -> &mut Self {
        self.joins.push((table, join, field));
        self
    }

    pub fn where_condition(&mut self, field: &str, condition: MySQLCondition) -> &mut Self {
        self.where_condition.push((field.to_string(), condition));
        self
    }

    pub fn group_by(&mut self, fields: Vec<String>) -> &mut Self {
        self.group_by = fields;
        self
    }

    pub fn having(&mut self, field: String, condition: MySQLCondition, value: String) -> &mut Self {
        self.having.push((field, condition, value));
        self
    }

    pub fn order_by(&mut self, field: String, order: MySQLOrder) -> &mut Self {
        self.order_by.push((field, order));
        self
    }

    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn build(&self) -> String {
        let mut query = self.action.to_string(&self.table);
        if self.joins.len() > 0 {
            for join in &self.joins {
                query = format!("{} {} JOIN {} ON {}", query, match join.1 {
                    MySQLJoin::Inner => "INNER",
                    MySQLJoin::Left => "LEFT",
                    MySQLJoin::Right => "RIGHT",
                    MySQLJoin::Full => "FULL",
                }, join.0, join.2);
            }
        }
        if self.where_condition.len() > 0 {
            query = format!("{} WHERE", query);
            for (index, condition) in self.where_condition.iter().enumerate() {
                query = format!("{} {} {} {}", query, condition.0, self.condition_builder(&condition.1), if index < self.where_condition.len() - 1 { "AND" } else { "" });
            }
        }
        if self.group_by.len() > 0 {
            query = format!("{} GROUP BY {}", query, self.group_by.join(", "));
        }

        if self.having.len() > 0 {
            query = format!("{} HAVING", query);
            for (index, condition) in self.having.iter().enumerate() {
                query = format!("{} {} {} {}", query, condition.0, self.condition_builder(&condition.1), if index < self.having.len() - 1 { "AND" } else { "" });
            }
        }

        if self.order_by.len() > 0 {
            query = format!("{} ORDER BY ", query);
            for (index, order) in self.order_by.iter().enumerate() {
                query = format!("{} {} {}", query, order.0, match order.1 {
                    MySQLOrder::Asc => "ASC",
                    MySQLOrder::Desc => "DESC",
                });
                if index < self.order_by.len() - 1 {
                    query = format!("{}, ", query);
                }
            }
        }

        if let Some(limit) = self.limit {
            query = format!("{} LIMIT {}", query, limit);
        }

        query.trim().to_string()


    }

    pub fn reset(&mut self) -> &mut Self {
        self.action = MySQlAction::None;
        self.joins = Vec::new();
        self.where_condition = Vec::new();
        self.group_by = Vec::new();
        self.having = Vec::new();
        self.order_by = Vec::new();
        self.limit = None;
        self
    }

    fn condition_builder(&self, condition: &MySQLCondition) -> String {
        condition.to_string(self.prepared_stmt)
    }
}