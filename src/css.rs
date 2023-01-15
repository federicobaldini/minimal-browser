pub type Specificity = (usize, usize, usize);

pub struct SimpleSelector {
  tag_name: Option<String>,
  id: Option<String>,
  class_list: Vec<String>,
}

impl SimpleSelector {
  pub fn new(
    tag_name: Option<String>,
    id: Option<String>,
    class_list: Vec<String>,
  ) -> SimpleSelector {
    SimpleSelector {
      tag_name,
      id,
      class_list,
    }
  }
  pub fn set_tag_name(&mut self, tag_name: Option<String>) {
    self.tag_name = tag_name;
  }
  pub fn set_id(&mut self, id: Option<String>) {
    self.id = id;
  }
  pub fn add_class(&mut self, class: String) {
    self.class_list.push(class);
  }
}

pub enum Selector {
  Simple(SimpleSelector),
}

impl Selector {
  pub fn specificity(&self) -> Specificity {
    // http://www.w3.org/TR/selectors/#specificity
    let Selector::Simple(ref simple) = *self;
    let a = simple.id.iter().count();
    let b = simple.class_list.len();
    let c = simple.tag_name.iter().count();
    (a, b, c)
  }
}

pub enum Value {
  Keyword(String),
  Length(f32, Unit),
  ColorValue(Color),
  // insert more values here
}

pub enum Unit {
  Px,
  // insert more units here
}

pub struct Color {
  red: u8,
  green: u8,
  blue: u8,
  alpha: u8,
}

impl Color {
  pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
    Color {
      red,
      green,
      blue,
      alpha,
    }
  }
}

pub struct Declaration {
  name: String,
  value: Value,
}

impl Declaration {
  pub fn new(name: String, value: Value) -> Declaration {
    Declaration { name, value }
  }
}

pub struct Rule {
  selectors: Vec<Selector>,
  declarations: Vec<Declaration>,
}

impl Rule {
  pub fn new(selectors: Vec<Selector>, declarations: Vec<Declaration>) -> Rule {
    Rule {
      selectors,
      declarations,
    }
  }
}

pub struct Stylesheet {
  rules: Vec<Rule>,
}

impl Stylesheet {
  pub fn new(rules: Vec<Rule>) -> Stylesheet {
    Stylesheet { rules }
  }
}