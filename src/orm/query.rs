use super::querybuilder::QueryBuilder;

#[derive(Debug, Clone, PartialEq)]
pub enum BindType {
    Int(i32),
    String(String),
    Bool(bool),
    Float(f32),
    PreparedStatement,
}

// impl std::fmt::Display for bind type 
impl std::fmt::Display for BindType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BindType::Int(value) => write!(f, "{}", value),
            BindType::String(value) => write!(f, "{}", value),
            BindType::Bool(value) => write!(f, "{}", value),
            BindType::Float(value) => write!(f, "{}", value),
            BindType::PreparedStatement => write!(f, "?"),
        }
    }
}

pub struct Query {
    pub query : String, 
    pub bind_values : Vec<BindType>,
    query_builder: QueryBuilder,

}


impl Query {
    pub fn new(query: QueryBuilder) -> Self {
        Self {
            query: query.build(),
            bind_values: Vec::new(),
            query_builder: query,
        }
    }

    pub fn bind(&mut self, value: BindType) {
        self.bind_values.push(value);
    }

    pub fn prepare(&mut self) -> sqlx::query::Query<'_, sqlx::MySql, sqlx::mysql::MySqlArguments> {
        self.query = self.query_builder.build();
        return self.ready_query();
    }

    pub fn ready_query(&self)  -> sqlx::query::Query<'_, sqlx::MySql, sqlx::mysql::MySqlArguments> {
        let mut stmt: sqlx::query::Query<'_, sqlx::MySql, sqlx::mysql::MySqlArguments> = sqlx::query::<sqlx::MySql>(self.query.as_str());
        for bind_value in &self.bind_values {
            match bind_value {
                BindType::Int(value) => {
                    stmt = stmt.bind(value);
                }
                BindType::String(value) => {
                    stmt = stmt.bind(value);
                }
                BindType::Bool(value) => {
                    stmt = stmt.bind(value);
                }
                BindType::Float(value) => {
                    stmt = stmt.bind(value);
                }
                _ => {}
            }
        }
        return stmt;
    }
    
}